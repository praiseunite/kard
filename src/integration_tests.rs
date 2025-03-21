// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, Addr, Uint128, CosmosMsg, BankMsg};

//     #[test]
//     fn test_redeem_gift() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();
//         let sender = "alice".to_string();
//         let redeemer = Addr::unchecked("bob");
//         let amount = 100_000u128;
//         let expiry = env.block.time.seconds() + 3600; // 1 hour expiry

//         // Simulate signing the message
//         let message_hash = format!("{}:{}:{}", sender, amount, expiry);
//         let signature = "valid_signature"; // Mock signature

//         // Bob redeems the card
//         let info = mock_info(&redeemer.to_string(), &[]);
//         let res = redeem_gift(
//             deps.as_mut(),
//             env.clone(),
//             info,
//             sender.clone(),
//             amount,
//             expiry,
//             signature.to_string(),
//         );
//         assert!(res.is_ok());

//         // Ensure Bob got the funds
//         let bank_msg = res.unwrap().messages[0].msg.clone();
//         match bank_msg {
//             CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
//                 assert_eq!(to_address, redeemer.to_string());
//                 assert_eq!(amount[0].amount, Uint128::new(100_000));
//             }
//             _ => panic!("Expected Bank Send Message"),
//         }
//     }
// }
