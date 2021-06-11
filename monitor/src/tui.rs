use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::{
    events::{Event, Events},
    gpu::GpuTable,
    task::TaskTable,
    util::TabsState,
    MonitorEvent,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::Backend,
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame, Terminal,
};

struct App<'a> {
    tabs: TabsState<'a>,
}

pub fn run_app(
    data_recv: Receiver<crate::MonitorEvent>,
    error_sender: Sender<MonitorEvent>,
) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App {
        tabs: TabsState::new(vec!["Gpus", "Tasks"]),
    };
    let mut table = GpuTable::new(7);
    let mut task_table = TaskTable::new();
    let mut service_down = true;
    let mut address = "".to_string();

    // Main loop
    loop {
        match data_recv.try_recv() {
            Ok(MonitorEvent::Abort) => unreachable!(),

            Err(TryRecvError::Disconnected) => {
                return Err(
                    "Channel pipe closed - data thread or server conection was closed"
                        .to_string()
                        .into(),
                )
            }
            Ok(MonitorEvent::NewData(info)) => {
                //update our data here
                service_down = false;
                table.update(&info);
                task_table.update(&info);
            }
            Ok(MonitorEvent::NoSchedulerService(addr)) => {
                address = addr;
                service_down = true;
            }
            Err(_) => {} // channel empty
        }

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default()
                .style(Style::default().fg(Color::White).bg(Color::Rgb(18, 11, 63)));
            f.render_widget(block, size);
            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::Yellow)),
                        Span::styled(rest, Style::default().fg(Color::Green)),
                    ])
                })
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);

            if service_down {
                draw_popup(f, chunks[1], address.clone());
            } else {
                match app.tabs.index {
                    0 => draw_gpu_table(f, &mut app, chunks[1], &mut table),
                    1 => draw_task_table(f, &mut app, chunks[1], &mut task_table),
                    _ => unreachable!(),
                };
            }
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    error_sender.send(MonitorEvent::Abort)?;
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Down => {
                    //table.next();
                }
                Key::Up => {
                    //table.previous();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn draw_popup<B>(f: &mut Frame<B>, area: Rect, text: String)
where
    B: Backend,
{
    let text = vec![Spans::from(Span::styled(
        format!("Scheduler service not running at: {}", text),
        Style::default().bg(Color::Red),
    ))];

    let block = Block::default().title("Error").borders(Borders::ALL);
    let paragraph = Paragraph::new(text.clone())
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    let area = centered_rect(60, 20, area);
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn draw_gpu_table<B>(f: &mut Frame<B>, _app: &mut App, area: Rect, table: &mut GpuTable)
where
    B: Backend,
{
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = [
        "Gpu",
        "Name",
        "Memory",
        "Mem-usage",
        "Busy",
        "Num-jobs",
        "Current-job",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = table.items.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(1u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("GPUs"))
        .highlight_style(selected_style)
        .column_spacing(2)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(30), //id
            Constraint::Percentage(20), //name
            Constraint::Percentage(10), //memory
            Constraint::Percentage(10), //memusage
            Constraint::Percentage(4),  //busy
            Constraint::Percentage(8),  //num-jobs
            Constraint::Percentage(13), //current
        ]);
    f.render_stateful_widget(t, area, &mut table.state);
}

fn draw_task_table<B>(f: &mut Frame<B>, _app: &mut App, area: Rect, table: &mut TaskTable)
where
    B: Backend,
{
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Job", "Gpus", "end", "last-seen"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = table
        .items
        .iter()
        .map(|item| {
            let stalled = item.job_id.chars().any(|c| c == '*');

            // job_id cell
            let id_cell = if stalled {
                Cell::from(item.job_id.clone()).style(Style::default().fg(Color::Red))
            } else {
                Cell::from(item.job_id.clone())
            };

            let row_hight = item.devices.len() + 1;

            let devices_cell = {
                let mut ids = String::new();
                for id in item.devices.iter() {
                    ids.push_str(format!("{}\n", id).as_str());
                }
                Cell::from(ids)
            };

            let end_cell = Cell::from(item.end.clone());
            let last_seen = Cell::from(item.last_seen.clone());
            Row::new(vec![id_cell, devices_cell, end_cell, last_seen]).height(row_hight as u16)
        })
        .collect::<Vec<_>>();

    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(selected_style)
        .column_spacing(1)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(24),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
        ]);
    f.render_stateful_widget(t, area, &mut table.state);
}
