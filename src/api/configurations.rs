use envconfig::Envconfig;

#[derive(Envconfig, Clone, Debug)]
pub struct ApiConfiguration {
    #[envconfig(from = "HTTP_PORT", default = "80")]
    pub http_port: u16,
}
