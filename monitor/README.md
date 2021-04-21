# Monitor tool

## Monitor

This is a simple tool for monitoring the Scheduler service. It does not require any dependency so far.

In order to start the monitor service run the following command:

```bash
cargo run --release -- -a 127.0.0.1:5000 monitor -r 200
```

- -a stands for *address*, It is the address of the scheduler service to connect to
- -r is the rate in milliseconds at which the monitor service will pull for data in order to update the info being displayed on the screen.

The monitor screen contains two tabs, the first one shows the systems GPUs and their current status like how much memory is being used and how many jobs are using it. The last tab contains relevant information about jobs in general. You can navigate between jobs using the __right-left__ arrow keys.

Once the service has been started, it can be stopped at any time by pressing the __q__ key.

## Abort

The tool also comes with a command argument that users can use to abort an undesired job in the scheduler. The job must be running.

The following command will abort the execution of job identified by 23:

```bash
cargo run --release -- -a 127.0.0.1:5000 abort -j 23
```

jobs are not canceled immediately and maybe still visible on the monitor-tool screen. It is so because jobs are only interrupted at a certain point in their execution flow.
