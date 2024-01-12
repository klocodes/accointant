use actix::prelude::*;
use crate::config::actor_error::ActorError;
use crate::config::Config;

pub struct ConfigActor {
    config: Config,
}

impl ConfigActor {
    pub fn new(config: Config) -> Self {
        ConfigActor { config }
    }
}

impl Actor for ConfigActor {
    type Context = Context<Self>;
    // Дополнительные методы и обработчики, если необходимы
}

// Определение сообщения для получения конфигурации
pub struct GetConfig;

impl Message for GetConfig {
    type Result = Result<Config, ActorError>; // Используйте подходящий тип ошибки
}

// Обработка сообщения в акторе
impl Handler<GetConfig> for ConfigActor {
    type Result = Result<Config, ActorError>; // Используйте подходящий тип ошибки

    fn handle(&mut self, _msg: GetConfig, _ctx: &mut Self::Context) -> Self::Result {
        Ok(self.config.clone()) // Возвращает копию конфигурации
    }
}
