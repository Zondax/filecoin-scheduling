pub struct SchedulerConfig {
    _address: String,
}

pub struct ClientConfig {}

pub struct Config {
    pub scheduler_cfg: SchedulerConfig,
    pub client_cfg: ClientConfig,
}
