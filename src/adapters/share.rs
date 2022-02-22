use std::ops::DerefMut;

use web_sys::ShareData;

use crate::components::ui::{Alert, Messages};

pub fn navigator_share(
    text: &str,
    url: &str,
    alert: &mut impl DerefMut<Target = Alert>,
    messages: &Messages,
) {
    unsafe {
        let text = &format!("{} {}", text, messages.tags);
        let url = &format!("https://hug.hihaheho.com{}", url);

        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let mut data = ShareData::new();
        data.title("The Hug");
        data.text(text);
        data.url(url);

        #[cfg(web_sys_unstable_apis)]
        {
            let mut clipboard = navigator.clipboard();
            let _ = clipboard.write_text(&format!("{}\n{}", text, url));
        }
        if js_sys::Reflect::get(&navigator, &"share".into())
            .unwrap()
            .is_function()
        {
            let _ = navigator.share(data);
        } else {
            alert.0 = messages.copied.into();
        }
    }
}
