use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD").expect("É esperado um token no ambiente");

    let intents = serenity::GatewayIntents::non_privileged();

    let options = poise::FrameworkOptions {
        commands: vec![ping()],
        ..Default::default()
    };

    if let Err(why) = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .options(options)
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Logado como {}!", ready.user.name);

                poise::builtins::register_globally(&ctx.http, &framework.options().commands)
                    .await?;

                println!("Comandos registrados globalmente!");
                Ok(Data {})
            })
        })
        .run()
        .await
    {
        println!("Erro ao iniciar o cliente: {:?}", why);
    }
}
