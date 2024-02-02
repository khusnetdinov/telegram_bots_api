use crate::api::params::{
    DeleteWebhookParams, ForwardMessageParams, ForwardMessagesParams, GetUpdateParams,
    SendMessageParams, SetWebhookParams,
};
use crate::api::types::message::Message;
use crate::api::types::message_id::MessageId;
use crate::api::types::update::Update;
use crate::api::types::user::User;
use crate::api::types::webhook_info::WebhookInfo;
use crate::errors::Error;

pub trait Requests {
    /// https://core.telegram.org/bots/api#getupdates
    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    fn get_updates(&self, params: &GetUpdateParams) -> Result<Vec<Update>, Error>;

    /// https://core.telegram.org/bots/api#setwebhook
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
    /// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    fn set_webhook(&self, params: &SetWebhookParams) -> Result<bool, Error>;

    /// https://core.telegram.org/bots/api#deletewebhook
    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    fn delete_webhook(&self, params: &DeleteWebhookParams) -> Result<bool, Error>;

    /// https://core.telegram.org/bots/api#getwebhookinfo
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    fn get_webhook_info(&self) -> Result<WebhookInfo, Error>;

    /// https://core.telegram.org/bots/api#getme
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    fn get_me(&self) -> Result<User, Error>;

    /// https://core.telegram.org/bots/api#logout
    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
    fn log_out(&self) -> Result<bool, Error>;

    /// https://core.telegram.org/bots/api#close
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    fn close(&self) -> Result<bool, Error>;

    /// https://core.telegram.org/bots/api#sendmessage
    /// Use this method to send text messages. On success, the sent Message is returned.
    fn send_message(&self, params: &SendMessageParams) -> Result<Message, Error>;

    /// https://core.telegram.org/bots/api#forwardmessage
    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
    fn forward_message(&self, params: &ForwardMessageParams) -> Result<MessageId, Error>;

    /// https://core.telegram.org/bots/api#forwardmessages
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
    fn forward_messages(&self, params: &ForwardMessagesParams) -> Result<MessageId, Error>;

    // // https://core.telegram.org/bots/api#copymessage
    // fn copy_message(&self)

    // // https://core.telegram.org/bots/api#copymessages
    // fn copy_messages(&self)

    // // https://core.telegram.org/bots/api#sendphoto
    // fn send_photo(&self)

    // // https://core.telegram.org/bots/api#sendaudio
    // fn send_audio(&self)

    // // https://core.telegram.org/bots/api#senddocument
    // fn send_document(&self)

    // // https://core.telegram.org/bots/api#sendvideo
    // fn send_video(&self)

    // // https://core.telegram.org/bots/api#sendanimation
    // fn send_animation(&self)

    // // https://core.telegram.org/bots/api#sendvoice
    // fn send_voice(&self)

    // // https://core.telegram.org/bots/api#sendvideonote
    // fn send_video_note(&self)

    // // https://core.telegram.org/bots/api#sendmediagroup
    // fn send_media_group(&self)

    // // https://core.telegram.org/bots/api#sendlocation
    // fn send_location(&self)

    // // https://core.telegram.org/bots/api#sendvenue
    // fn send_venue(&self)

    // // https://core.telegram.org/bots/api#sendcontact
    // fn send_contact(&self)

    // // https://core.telegram.org/bots/api#sendpoll
    // fn send_poll(&self)

    // // https://core.telegram.org/bots/api#senddice
    // fn send_dice(&self)

    // // https://core.telegram.org/bots/api#sendchataction
    // fn send_chat_action(&self)

    // // https://core.telegram.org/bots/api#setmessagereaction
    // fn set_message_reaction(&self)

    // // https://core.telegram.org/bots/api#getuserprofilephotos
    // fn get_user_profile_photos(&self)

    // // https://core.telegram.org/bots/api#getfile
    // fn get_file(&self)

    // // https://core.telegram.org/bots/api#banchatmember
    // fn ban_chat_member(&self)

    // // https://core.telegram.org/bots/api#unbanchatmember
    // fn unban_chat_member(&self)

    // // https://core.telegram.org/bots/api#restrictchatmember
    // fn restrict_chat_member(&self)

    // // https://core.telegram.org/bots/api#promotechatmember
    // fn promote_chat_member(&self)

    // // https://core.telegram.org/bots/api#setchatadministratorcustomtitle
    // fn set_chat_administrator_custom_title(&self)

    // // https://core.telegram.org/bots/api#banchatsenderchat
    // fn ban_chat_sender_chat(&self)

    // // https://core.telegram.org/bots/api#unbanchatsenderchat
    // fn unban_chat_sender_chat(&self)

    // // https://core.telegram.org/bots/api#setchatpermissions
    // fn set_chat_permissions(&self)

    // // https://core.telegram.org/bots/api#exportchatinvitelink
    // fn export_chat_invite_link(&self)

    // // https://core.telegram.org/bots/api#createchatinvitelink
    // fn create_chat_invite_link(&self)

    // // https://core.telegram.org/bots/api#editchatinvitelink
    // fn edit_chat_invite_link(&self)

    // // https://core.telegram.org/bots/api#revokechatinvitelink
    // fn revoke_chat_invite_link(&self)

    // // https://core.telegram.org/bots/api#approvechatjoinrequest
    // fn approve_chat_join_request(&self)

    // // https://core.telegram.org/bots/api#declinechatjoinrequest
    // fn decline_chat_join_request(&self)

    // // https://core.telegram.org/bots/api#setchatphoto
    // fn set_chat_photo(&self)

    // // https://core.telegram.org/bots/api#deletechatphoto
    // fn delete_chat_photo(&self)

    // // https://core.telegram.org/bots/api#setchattitle
    // fn set_chat_title(&self)

    // // https://core.telegram.org/bots/api#setchatdescription
    // fn set_chat_description(&self)

    // // https://core.telegram.org/bots/api#pinchatmessage
    // fn pin_chat_message(&self)

    // // https://core.telegram.org/bots/api#unpinchatmessage
    // fn unpin_chat_message(&self)

    // // https://core.telegram.org/bots/api#unpinallchatmessages
    // fn unpin_all_chat_messages(&self)

    // // https://core.telegram.org/bots/api#leavechat
    // fn leave_chat(&self)

    // // https://core.telegram.org/bots/api#getchat
    // fn get_chat(&self)

    // // https://core.telegram.org/bots/api#getchatadministrators
    // fn get_chat_administrators(&self)

    // // https://core.telegram.org/bots/api#getchatmembercount
    // fn get_chat_member_count(&self)

    // // https://core.telegram.org/bots/api#getchatmember
    // fn get_chat_member(&self)

    // // https://core.telegram.org/bots/api#setchatstickerset
    // fn set_chat_sticker_set(&self)

    // // https://core.telegram.org/bots/api#deletechatstickerset
    // fn delete_chat_sticker_set(&self)

    // // https://core.telegram.org/bots/api#getforumtopiciconstickers
    // fn get_forum_topic_icon_stickers(&self)

    // // https://core.telegram.org/bots/api#createforumtopic
    // fn create_forum_topic(&self)

    // // https://core.telegram.org/bots/api#editforumtopic
    // fn edit_forum_topic(&self)

    // // https://core.telegram.org/bots/api#closeforumtopic
    // fn close_forum_topic(&self)

    // // https://core.telegram.org/bots/api#reopenforumtopic
    // fn reopen_forum_topic(&self)

    // // https://core.telegram.org/bots/api#deleteforumtopic
    // fn delete_forum_topic(&self)

    // // https://core.telegram.org/bots/api#unpinallforumtopicmessages
    // fn unpin_all_forum_topic_messages(&self)

    // // https://core.telegram.org/bots/api#editgeneralforumtopic
    // fn edit_general_forum_topic(&self)

    // // https://core.telegram.org/bots/api#closegeneralforumtopic
    // fn close_general_forum_topic(&self)

    // // https://core.telegram.org/bots/api#reopengeneralforumtopic
    // fn reopen_general_forum_topic(&self)

    // // https://core.telegram.org/bots/api#hidegeneralforumtopic
    // fn hide_general_forum_topic(&self)

    // // https://core.telegram.org/bots/api#unhidegeneralforumtopic
    // fn unhide_general_forum_topic(&self)

    // // https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
    // fn unpin_all_general_forum_topic_messages(&self)

    // // https://core.telegram.org/bots/api#answercallbackquery
    // fn answer_callback_query(&self)

    // // https://core.telegram.org/bots/api#getuserchatboosts
    // fn get_user_chat_boosts(&self)

    // // https://core.telegram.org/bots/api#setmycommands
    // fn set_my_commands(&self)

    // // https://core.telegram.org/bots/api#deletemycommands
    // fn delete_my_commands(&self)

    // // https://core.telegram.org/bots/api#getmycommands
    // fn get_my_commands(&self)

    // // https://core.telegram.org/bots/api#setmyname
    // fn set_my_name(&self)

    // // https://core.telegram.org/bots/api#getmyname
    // fn get_my_name(&self)

    // // https://core.telegram.org/bots/api#setmydescription
    // fn set_my_description(&self)

    // // https://core.telegram.org/bots/api#getmydescription
    // fn get_my_description(&self)

    // // https://core.telegram.org/bots/api#setmyshortdescription
    // fn set_my_short_description(&self)

    // // https://core.telegram.org/bots/api#getmyshortdescription
    // fn get_my_short_description(&self)

    // // https://core.telegram.org/bots/api#setchatmenubutton
    // fn set_chat_menu_button(&self)

    // // https://core.telegram.org/bots/api#getchatmenubutton
    // fn get_chat_menu_button(&self)

    // // https://core.telegram.org/bots/api#setmydefaultadministratorrights
    // fn set_my_default_administrator_rights(&self)

    // // https://core.telegram.org/bots/api#getmydefaultadministratorrights
    // fn get_my_default_administrator_rights(&self)

    // // https://core.telegram.org/bots/api#editmessagetext
    // fn edit_message_text(&self)

    // // https://core.telegram.org/bots/api#editmessagecaption
    // fn edit_message_caption(&self)

    // // https://core.telegram.org/bots/api#editmessagemedia
    // fn edit_message_media(&self)

    // // https://core.telegram.org/bots/api#editmessagelivelocation
    // fn edit_message_live_location(&self)

    // // https://core.telegram.org/bots/api#stopmessagelivelocation
    // fn stop_message_live_location(&self)

    // // https://core.telegram.org/bots/api#editmessagereplymarkup
    // fn edit_message_reply_markup(&self)

    // // https://core.telegram.org/bots/api#stoppoll
    // fn stop_poll(&self)

    // // https://core.telegram.org/bots/api#deletemessage
    // fn delete_message(&self)

    // // https://core.telegram.org/bots/api#deletemessages
    // fn delete_messages(&self)

    // // https://core.telegram.org/bots/api#sendsticker
    // fn send_sticker(&self)

    // // https://core.telegram.org/bots/api#getstickerset
    // fn get_sticker_set(&self)

    // // https://core.telegram.org/bots/api#getcustomemojistickers
    // fn get_custom_emoji_stickers(&self)

    // // https://core.telegram.org/bots/api#uploadstickerfile
    // fn upload_sticker_file(&self)

    // // https://core.telegram.org/bots/api#createnewstickerset
    // fn create_new_sticker_set(&self)

    // // https://core.telegram.org/bots/api#addstickertoset
    // fn add_sticker_to_set(&self)

    // // https://core.telegram.org/bots/api#setstickerpositioninset
    // fn set_sticker_position_in_set(&self)

    // // https://core.telegram.org/bots/api#deletestickerfromset
    // fn delete_sticker_from_set(&self)

    // // https://core.telegram.org/bots/api#setstickeremojilist
    // fn set_sticker_emoji_list(&self)

    // // https://core.telegram.org/bots/api#setstickerkeywords
    // fn set_sticker_keywords(&self)

    // // https://core.telegram.org/bots/api#setstickermaskposition
    // fn set_sticker_mask_position(&self)

    // // https://core.telegram.org/bots/api#setstickersettitle
    // fn set_sticker_set_title(&self)

    // // https://core.telegram.org/bots/api#setstickersetthumbnail
    // fn set_sticker_set_thumbnail(&self)

    // // https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
    // fn set_custom_emoji_sticker_set_thumbnail(&self)

    // // https://core.telegram.org/bots/api#deletestickerset
    // fn delete_sticker_set(&self)

    // // https://core.telegram.org/bots/api#answerinlinequery
    // fn answer_inline_query(&self)

    // // https://core.telegram.org/bots/api#answerwebappquery
    // fn answer_web_app_query(&self)

    // // https://core.telegram.org/bots/api#sendinvoice
    // fn send_invoice(&self)

    // // https://core.telegram.org/bots/api#createinvoicelink
    // fn create_invoice_link(&self)

    // // https://core.telegram.org/bots/api#answershippingquery
    // fn answer_shipping_query(&self)

    // // https://core.telegram.org/bots/api#answerprecheckoutquery
    // fn answer_pre_checkout_query(&self)

    // // https://core.telegram.org/bots/api#setpassportdataerrors
    // fn set_passport_data_errors(&self)

    // // https://core.telegram.org/bots/api#sendgame
    // fn send_game(&self)

    // // https://core.telegram.org/bots/api#setgamescore
    // fn set_game_score(&self)

    // // https://core.telegram.org/bots/api#getgamehighscores
    // fn get_game_high_scores(&self)
}
