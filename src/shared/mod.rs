use anyhow::Result;
use async_trait::async_trait;

//#[async_trait]
//pub(crate) trait CommandResponseHandler<TCommand, TCommandResponse> {
//    async fn handle(&self, command: TCommand) -> Result<TCommandResponse>;
//}

#[async_trait]
pub(crate) trait CommandHandler<TCommand> {
    async fn handle(&self, command: TCommand) -> Result<()>;
}
