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
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table, Tabs},
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
        tabs: TabsState::new(vec!["Gpus", "Taks"]),
    };
    let mut table = GpuTable::new(7);
    let mut task_table = TaskTable::new();

    // Main loop
    loop {
        match data_recv.try_recv() {
            Ok(MonitorEvent::Abort) => return Err("Client abort".to_string().into()),

            Err(TryRecvError::Disconnected) => {
                return Err(
                    "Channel pipe closed - datat thread was closed or server connection"
                        .to_string()
                        .into(),
                )
            }
            Ok(MonitorEvent::NewData(info)) => {
                //update our data here
                table.update(&info);
                task_table.update(&info);
            }
            Err(_) => {} // no data
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
            match app.tabs.index {
                0 => draw_gpu_table(f, &mut app, chunks[1], &mut table),
                1 => draw_task_table(f, &mut app, chunks[1], &mut task_table),
                _ => unreachable!(),
            };
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

fn draw_gpu_table<B>(f: &mut Frame<B>, _app: &mut App, area: Rect, table: &mut GpuTable)
where
    B: Backend,
{
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    // gpu, name, memory, in_use, is_busy, num_jobs, current_job
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
        .column_spacing(1)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
        ]);
    f.render_stateful_widget(t, area, &mut table.state);
}

fn draw_task_table<B>(f: &mut Frame<B>, _app: &mut App, area: Rect, table: &mut TaskTable)
where
    B: Backend,
{
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    // job, gpus, end, last_seen
    let header_cells = ["Job", "Gpus", "end", "last-seen"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = table.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '*').count())
            .max()
            .unwrap_or(0);

        let cells = item.iter().map(|c| Cell::from(c.clone()));
        // red for stalled jobs
        if height > 0 {
            Row::new(cells)
                .height(1u16)
                .bottom_margin(1)
                .style(Style::default().fg(Color::Red))
        } else {
            Row::new(cells).height(1u16).bottom_margin(1)
        }
    });
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
