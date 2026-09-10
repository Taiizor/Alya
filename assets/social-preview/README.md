# Social Preview

The repository's social media preview card for **Alya** (the image shown when the repo is shared on GitHub, X, Discord, Slack, etc.).

- `social-card.html` — The self-contained source (inline CSS, no external fonts or assets required). Edit this file to customize the card design.
- `social-card.png` — The rendered card, exactly **1280×640 px** (GitHub's recommended aspect ratio).

---

## Setting the Social Preview on GitHub

1. Navigate to your repository on GitHub: [github.com/alya-lang/alya](https://github.com/alya-lang/alya)
2. Go to **Settings** → **General**
3. Scroll to **Social preview**
4. Click **Edit** → **Upload an image…**
5. Select `assets/social-preview/social-card.png` (or drag and drop it)

---

## Regenerating the PNG after Editing the HTML

Render the HTML into an exact 1280×640 PNG using headless Chrome or Edge:

### PowerShell (Windows)

```powershell
Start-Process -FilePath "C:\Program Files\Google\Chrome\Application\chrome.exe" -ArgumentList @(
  "--headless=new",
  "--hide-scrollbars",
  "--force-device-scale-factor=1",
  "--window-size=1280,640",
  "--screenshot=$PWD\assets\social-preview\social-card.png",
  "file:///$($PWD.Path.Replace('\','/'))/assets/social-preview/social-card.html"
) -Wait -NoNewWindow
```

*Note: You can also use `msedge.exe` (Microsoft Edge) with the same flags.*

### Bash (Linux / macOS)

```bash
chrome --headless=new --hide-scrollbars --force-device-scale-factor=1 \
  --window-size=1280,640 \
  --screenshot=assets/social-preview/social-card.png \
  "file://$PWD/assets/social-preview/social-card.html"
```
