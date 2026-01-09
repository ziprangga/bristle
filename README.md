# [![bristapp Icon](doc/icon/bristapp_64.png)](doc/icon/bristapp_64.png) **BristApp**

## Screenshot

<img width="500" alt="bristapp UI" src="doc/bristapp_1.png" /> <img width="500" alt="bristapp UI" src="doc/bristapp_2.png" />

---

**BristApp** is a cleanup tool designed to safely remove applications and their associated files or folders from the macOS system.

This app was built using open-source components and inspired by privacy guides from **[Sunknudsen](https://github.com/sunknudsen/guides/tree/main/archive/how-to-clean-uninstall-macos-apps-using-appcleaner-open-source-alternative)**. Some of the scripts in this app adapt references from his guides, so I want to give him a big thank you!

The app’s interface is kept simple using Rust with gui using iced rs, with the main goal of helping my beloved wife and friends who rarely use the terminal on macOS.
On top of that, this project also serves as a way for me to dive deeper into Rust.

#### "Inspect and compile it"

---

## Permissions and Privacy Notice for macOS

To perform cleanup app effectively, **BristApp** requires specific permissions when running on macOS. Below is a detailed explanation:

### Access to Finder (Automation Permission)

BristApp interacts with Finder to:

- 🗑️ **Move selected files or folders to the Trash.**

**This permission is required.**  
Without this permission, BristApp cannot move files or folders to the Trash.

**How to Grant Finder Access:**

1. Go to **System Preferences → Security & Privacy → Privacy → Automation**.
2. Ensure **BristApp** is allowed to control Finder.
3. Restart the application after granting this permission.

---

### Why These Permissions Are Needed

Permissions are strictly used for:

- **Locating and displaying file paths** related to an application.
- Allowing you to **open file locations** directly in Finder.
- Securely **moving files or folders to the Trash**.

**No files will be deleted automatically** — all actions require user confirmation.

---

### Troubleshooting Permissions

If you encounter issues (e.g., files not moving to Trash), follow these steps:

1. Open **System Preferences → Security & Privacy → Privacy**.
2. Under **Automation**, ensure **BristApp** has permission to control Finder.
3. Restart BristApp after granting the required permissions.

---

## How to Use BristApp

BristApp makes cleaning up applications simple and intuitive. Follow these steps:

### Selecting an Application

- **Drag & Drop**: Drag the application you want to clean into the BristApp window.
- **Export Bom Logs**: Use to export bom logs file, it can be use for more advanced inspection manually (you can watch Sunknudsen explain).

---

### Displaying Related Files or Folders

Once an application is selected, BristApp will display a list of related files or folders.

- **Delete All**: Click the **Move to Trash** button to move all files/folders to the Trash.

---

### Verifying Deleted Files

Files or folders moved to the **Trash** can be reviewed. If needed, you can restore them to their original location.

---

### Opening File/Folder Locations

To open the location of a file or folder:

- **click** on the item in the path list name.

---

### Searching for Log Files (BOM File Log)

BristApp can also search for log files to help with more thorough cleanup.

- **Default Location**: Log files are automatically saved to the **Desktop** but can be replace in input field.

---

## License

Licensed under either of:

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

---

## Contributing

Contributions are welcome! If you'd like to improve BristApp or add new features, please open an issue or submit a pull request.

---

## Need Help?

If you experience issues or have questions, please check the **[Wiki](https://github.com/ziprangga/bristapp/wiki)** or open an issue.
