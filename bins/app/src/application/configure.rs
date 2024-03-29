
pub async fn configure(){

    tokio::spawn(async {
        web_app_config().await;
    });
}


pub async fn web_app_config(){
    super::web_app::web_app_main::web_app_main().await;
}