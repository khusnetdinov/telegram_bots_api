use crate::api::types::user::User;
use crate::errors::Error;
use std::future::Future;

pub trait Requests {
    // // https://core.telegram.org/bots/api#getupdates
    // async fn get_updates(&self);

    // // https://core.telegram.org/bots/api#setwebhook
    // async fn set_webhook(&self);

    // // https://core.telegram.org/bots/api#deletewebhook
    // async fn delete_webhook(&self);

    // // https://core.telegram.org/bots/api#getwebhookinfo
    // async fn get_webhook_info(&self);

    /// https://core.telegram.org/bots/api#getme
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    fn get_me(&self) -> impl Future<Output = Result<User, Error>>;

    // // https://core.telegram.org/bots/api#logout
    // async fn log_out(&self);

    // // https://core.telegram.org/bots/api#close
    // async fn close(&self);

    // // https://core.telegram.org/bots/api#sendmessage
    // async fn send_message(&self);

    // // https://core.telegram.org/bots/api#forwardmessage
    // async fn forward_message(&self);

    // // https://core.telegram.org/bots/api#forwardmessages
    // async fn forward_messages(&self);

    // // https://core.telegram.org/bots/api#copymessage
    // async fn copy_message(&self);

    // // https://core.telegram.org/bots/api#copymessages
    // async fn copy_messages(&self);

    // // https://core.telegram.org/bots/api#sendphoto
    // async fn send_photo(&self);

    // // https://core.telegram.org/bots/api#sendaudio
    // async fn send_audio(&self);

    // // https://core.telegram.org/bots/api#senddocument
    // async fn send_document(&self);

    // // https://core.telegram.org/bots/api#sendvideo
    // async fn send_video(&self);

    // // https://core.telegram.org/bots/api#sendanimation
    // async fn send_animation(&self);

    // // https://core.telegram.org/bots/api#sendvoice
    // async fn send_voice(&self);

    // // https://core.telegram.org/bots/api#sendvideonote
    // async fn send_video_note(&self);

    // // https://core.telegram.org/bots/api#sendmediagroup
    // async fn send_media_group(&self);

    // // https://core.telegram.org/bots/api#sendlocation
    // async fn send_location(&self);

    // // https://core.telegram.org/bots/api#sendvenue
    // async fn send_venue(&self);

    // // https://core.telegram.org/bots/api#sendcontact
    // async fn send_contact(&self);

    // // https://core.telegram.org/bots/api#sendpoll
    // async fn send_poll(&self);

    // // https://core.telegram.org/bots/api#senddice
    // async fn send_dice(&self);

    // // https://core.telegram.org/bots/api#sendchataction
    // async fn send_chat_action(&self);

    // // https://core.telegram.org/bots/api#setmessagereaction
    // async fn set_message_reaction(&self);

    // // https://core.telegram.org/bots/api#getuserprofilephotos
    // async fn get_user_profile_photos(&self);

    // // https://core.telegram.org/bots/api#getfile
    // async fn get_file(&self);

    // // https://core.telegram.org/bots/api#banchatmember
    // async fn ban_chat_member(&self);

    // // https://core.telegram.org/bots/api#unbanchatmember
    // async fn unban_chat_member(&self);

    // // https://core.telegram.org/bots/api#restrictchatmember
    // async fn restrict_chat_member(&self);

    // // https://core.telegram.org/bots/api#promotechatmember
    // async fn promote_chat_member(&self);

    // // https://core.telegram.org/bots/api#setchatadministratorcustomtitle
    // async fn set_chat_administrator_custom_title(&self);

    // // https://core.telegram.org/bots/api#banchatsenderchat
    // async fn ban_chat_sender_chat(&self);

    // // https://core.telegram.org/bots/api#unbanchatsenderchat
    // async fn unban_chat_sender_chat(&self);

    // // https://core.telegram.org/bots/api#setchatpermissions
    // async fn set_chat_permissions(&self);

    // // https://core.telegram.org/bots/api#exportchatinvitelink
    // async fn export_chat_invite_link(&self);

    // // https://core.telegram.org/bots/api#createchatinvitelink
    // async fn create_chat_invite_link(&self);

    // // https://core.telegram.org/bots/api#editchatinvitelink
    // async fn edit_chat_invite_link(&self);

    // // https://core.telegram.org/bots/api#revokechatinvitelink
    // async fn revoke_chat_invite_link(&self);

    // // https://core.telegram.org/bots/api#approvechatjoinrequest
    // async fn approve_chat_join_request(&self);

    // // https://core.telegram.org/bots/api#declinechatjoinrequest
    // async fn decline_chat_join_request(&self);

    // // https://core.telegram.org/bots/api#setchatphoto
    // async fn set_chat_photo(&self);

    // // https://core.telegram.org/bots/api#deletechatphoto
    // async fn delete_chat_photo(&self);

    // // https://core.telegram.org/bots/api#setchattitle
    // async fn set_chat_title(&self);

    // // https://core.telegram.org/bots/api#setchatdescription
    // async fn set_chat_description(&self);

    // // https://core.telegram.org/bots/api#pinchatmessage
    // async fn pin_chat_message(&self);

    // // https://core.telegram.org/bots/api#unpinchatmessage
    // async fn unpin_chat_message(&self);

    // // https://core.telegram.org/bots/api#unpinallchatmessages
    // async fn unpin_all_chat_messages(&self);

    // // https://core.telegram.org/bots/api#leavechat
    // async fn leave_chat(&self);

    // // https://core.telegram.org/bots/api#getchat
    // async fn get_chat(&self);

    // // https://core.telegram.org/bots/api#getchatadministrators
    // async fn get_chat_administrators(&self);

    // // https://core.telegram.org/bots/api#getchatmembercount
    // async fn get_chat_member_count(&self);

    // // https://core.telegram.org/bots/api#getchatmember
    // async fn get_chat_member(&self);

    // // https://core.telegram.org/bots/api#setchatstickerset
    // async fn set_chat_sticker_set(&self);

    // // https://core.telegram.org/bots/api#deletechatstickerset
    // async fn delete_chat_sticker_set(&self);

    // // https://core.telegram.org/bots/api#getforumtopiciconstickers
    // async fn get_forum_topic_icon_stickers(&self);

    // // https://core.telegram.org/bots/api#createforumtopic
    // async fn create_forum_topic(&self);

    // // https://core.telegram.org/bots/api#editforumtopic
    // async fn edit_forum_topic(&self);

    // // https://core.telegram.org/bots/api#closeforumtopic
    // async fn close_forum_topic(&self);

    // // https://core.telegram.org/bots/api#reopenforumtopic
    // async fn reopen_forum_topic(&self);

    // // https://core.telegram.org/bots/api#deleteforumtopic
    // async fn delete_forum_topic(&self);

    // // https://core.telegram.org/bots/api#unpinallforumtopicmessages
    // async fn unpin_all_forum_topic_messages(&self);

    // // https://core.telegram.org/bots/api#editgeneralforumtopic
    // async fn edit_general_forum_topic(&self);

    // // https://core.telegram.org/bots/api#closegeneralforumtopic
    // async fn close_general_forum_topic(&self);

    // // https://core.telegram.org/bots/api#reopengeneralforumtopic
    // async fn reopen_general_forum_topic(&self);

    // // https://core.telegram.org/bots/api#hidegeneralforumtopic
    // async fn hide_general_forum_topic(&self);

    // // https://core.telegram.org/bots/api#unhidegeneralforumtopic
    // async fn unhide_general_forum_topic(&self);

    // // https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
    // async fn unpin_all_general_forum_topic_messages(&self);

    // // https://core.telegram.org/bots/api#answercallbackquery
    // async fn answer_callback_query(&self);

    // // https://core.telegram.org/bots/api#getuserchatboosts
    // async fn get_user_chat_boosts(&self);

    // // https://core.telegram.org/bots/api#setmycommands
    // async fn set_my_commands(&self);

    // // https://core.telegram.org/bots/api#deletemycommands
    // async fn delete_my_commands(&self);

    // // https://core.telegram.org/bots/api#getmycommands
    // async fn get_my_commands(&self);

    // // https://core.telegram.org/bots/api#setmyname
    // async fn set_my_name(&self);

    // // https://core.telegram.org/bots/api#getmyname
    // async fn get_my_name(&self);

    // // https://core.telegram.org/bots/api#setmydescription
    // async fn set_my_description(&self);

    // // https://core.telegram.org/bots/api#getmydescription
    // async fn get_my_description(&self);

    // // https://core.telegram.org/bots/api#setmyshortdescription
    // async fn set_my_short_description(&self);

    // // https://core.telegram.org/bots/api#getmyshortdescription
    // async fn get_my_short_description(&self);

    // // https://core.telegram.org/bots/api#setchatmenubutton
    // async fn set_chat_menu_button(&self);

    // // https://core.telegram.org/bots/api#getchatmenubutton
    // async fn get_chat_menu_button(&self);

    // // https://core.telegram.org/bots/api#setmydefaultadministratorrights
    // async fn set_my_default_administrator_rights(&self);

    // // https://core.telegram.org/bots/api#getmydefaultadministratorrights
    // async fn get_my_default_administrator_rights(&self);

    // // https://core.telegram.org/bots/api#editmessagetext
    // async fn edit_message_text(&self);

    // // https://core.telegram.org/bots/api#editmessagecaption
    // async fn edit_message_caption(&self);

    // // https://core.telegram.org/bots/api#editmessagemedia
    // async fn edit_message_media(&self);

    // // https://core.telegram.org/bots/api#editmessagelivelocation
    // async fn edit_message_live_location(&self);

    // // https://core.telegram.org/bots/api#stopmessagelivelocation
    // async fn stop_message_live_location(&self);

    // // https://core.telegram.org/bots/api#editmessagereplymarkup
    // async fn edit_message_reply_markup(&self);

    // // https://core.telegram.org/bots/api#stoppoll
    // async fn stop_poll(&self);

    // // https://core.telegram.org/bots/api#deletemessage
    // async fn delete_message(&self);

    // // https://core.telegram.org/bots/api#deletemessages
    // async fn delete_messages(&self);

    // // https://core.telegram.org/bots/api#sendsticker
    // async fn send_sticker(&self);

    // // https://core.telegram.org/bots/api#getstickerset
    // async fn get_sticker_set(&self);

    // // https://core.telegram.org/bots/api#getcustomemojistickers
    // async fn get_custom_emoji_stickers(&self);

    // // https://core.telegram.org/bots/api#uploadstickerfile
    // async fn upload_sticker_file(&self);

    // // https://core.telegram.org/bots/api#createnewstickerset
    // async fn create_new_sticker_set(&self);

    // // https://core.telegram.org/bots/api#addstickertoset
    // async fn add_sticker_to_set(&self);

    // // https://core.telegram.org/bots/api#setstickerpositioninset
    // async fn set_sticker_position_in_set(&self);

    // // https://core.telegram.org/bots/api#deletestickerfromset
    // async fn delete_sticker_from_set(&self);

    // // https://core.telegram.org/bots/api#setstickeremojilist
    // async fn set_sticker_emoji_list(&self);

    // // https://core.telegram.org/bots/api#setstickerkeywords
    // async fn set_sticker_keywords(&self);

    // // https://core.telegram.org/bots/api#setstickermaskposition
    // async fn set_sticker_mask_position(&self);

    // // https://core.telegram.org/bots/api#setstickersettitle
    // async fn set_sticker_set_title(&self);

    // // https://core.telegram.org/bots/api#setstickersetthumbnail
    // async fn set_sticker_set_thumbnail(&self);

    // // https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
    // async fn set_custom_emoji_sticker_set_thumbnail(&self);

    // // https://core.telegram.org/bots/api#deletestickerset
    // async fn delete_sticker_set(&self);

    // // https://core.telegram.org/bots/api#answerinlinequery
    // async fn answer_inline_query(&self);

    // // https://core.telegram.org/bots/api#answerwebappquery
    // async fn answer_web_app_query(&self);

    // // https://core.telegram.org/bots/api#sendinvoice
    // async fn send_invoice(&self);

    // // https://core.telegram.org/bots/api#createinvoicelink
    // async fn create_invoice_link(&self);

    // // https://core.telegram.org/bots/api#answershippingquery
    // async fn answer_shipping_query(&self);

    // // https://core.telegram.org/bots/api#answerprecheckoutquery
    // async fn answer_pre_checkout_query(&self);

    // // https://core.telegram.org/bots/api#setpassportdataerrors
    // async fn set_passport_data_errors(&self);

    // // https://core.telegram.org/bots/api#sendgame
    // async fn send_game(&self);

    // // https://core.telegram.org/bots/api#setgamescore
    // async fn set_game_score(&self);

    // // https://core.telegram.org/bots/api#getgamehighscores
    // async fn get_game_high_scores(&self);
}
