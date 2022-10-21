use ft_logic_io::Action;
use ft_main_io::{FTokenAction, FTokenEvent};
use gstd::{msg, prelude::*, ActorId};

pub async fn transfer_tokens(
    transaction_id: u64,
    token_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *token_id,
        FTokenAction::Message {
            transaction_id,
            payload: Action::Transfer {
                sender: *sender,
                recipient: *recipient,
                amount,
            }
            .encode(),
        },
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(()),
    }
}

pub async fn balance(token_id: &ActorId, account: &ActorId) -> u128 {
    let reply =
        msg::send_for_reply_as::<_, FTokenEvent>(*token_id, FTokenAction::GetBalance(*account), 0)
            .expect("Error in sending a message `FTokenAction::GetBalance`")
            .await
            .expect("Unable to decode `FTokenEvent`");
    match reply {
        FTokenEvent::Balance(balance_response) => balance_response,
        _ => 0,
    }
}
