use serenity::{
    model::{
        channel::{
            Reaction,
            ReactionType
        },
        gateway::Ready,
    },
    prelude::{
        Context,
        EventHandler
    },
};

pub struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(err) = reaction.channel_id.say(
            &ctx.http,
            format!(
                "{} left a {} reaction",
                reaction.user(&ctx).unwrap().name,
                match reaction.emoji {
                    ReactionType::Custom {
                        animated: _animated,
                        id: _id,
                        name,
                    } => name.unwrap(),
                    ReactionType::Unicode(uni) => uni,
                    ReactionType::__Nonexhaustive => String::new(),
                }
            )
        ) {
            println!("Error reacting to a reaction: {:?}", err);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
