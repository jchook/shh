use std::process::Command;

pub fn send_system_notification() {
    #[cfg(target_os = "windows")]
    {
        // Use PowerShell to send a toast notification
        let ps_command = r#"
            [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null;
            $template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText01);
            $textNodes = $template.GetElementsByTagName('text');
            $textNodes.Item(0).AppendChild($template.CreateTextNode('Please be quiet, you are too loud!')) | Out-Null;
            $toast = [Windows.UI.Notifications.ToastNotification]::new($template);
            [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('Shh!').Show($toast);
        "#;

        // Execute PowerShell command
        let result = Command::new("powershell")
            .args(["-NoProfile", "-Command", ps_command])
            .output();

        // If it fails, log an error message
        if let Err(e) = result {
            eprintln!("Failed to send notification: {}", e);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Err(e) = Command::new("osascript")
            .arg("-e")
            .arg("display notification \"Please be quiet, you are too loud!\" with title \"Shh\"")
            .output()
        {
            eprintln!("Failed to send notification: {}", e);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = Command::new("notify-send")
            .arg("Shh")
            .arg("Please be quiet, you are too loud!")
            .output()
        {
            eprintln!("Failed to send notification: {}", e);
        }
    }
}
