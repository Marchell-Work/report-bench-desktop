# Report Bench desktop app (Windows)

A small installed Windows app that opens Report Bench in its own window, like Mobiloan.

- **Website changes** (anything you deploy to the Cloudflare worker) reach the app straight away.
  The site's own update banner (Build #18 and later) asks people to update, log out and back in.
- **App changes** (this shell itself, which is rare) are checked every time the app starts.
  If there's a newer version, people get a prompt: *Update now / Later*.

The app opens `https://reports.m-botha.workers.dev/`. To change that, edit `APP_URL` in
`src-tauri/src/main.rs`.

---

## One-time setup (about 15 minutes, all in the browser)

### 1. Create the GitHub repository
1. Sign in at github.com (create a free account if needed).
2. Click **+ → New repository**.
3. Name it exactly **report-bench-desktop**, set it to **Public**, and click **Create repository**.
   (Public is fine: the repo only holds the window shell. Your private signing key never goes into it.
   It must be public so installed apps can download updates without a login.)

### 2. (Done) Your GitHub username
Already set to **Marchell-Work** in `src-tauri/tauri.conf.json`. If the repo ever moves, update the
`endpoints` line there.

### 3. Upload the project files
1. In the new repo, click **uploading an existing file**.
2. Drag in everything from this folder **except** the `.github` folder:
   `src-tauri`, `ui`, `package.json`, `package-lock.json`, `.gitignore`, `README.md`.
3. Click **Commit changes**.
4. Add the workflow: click **Add file → Create new file**, type the name
   `.github/workflows/release.yml`, paste in the contents of that file from this folder,
   and click **Commit changes**.

### 4. Add the two signing secrets
In the repo, go to **Settings → Secrets and variables → Actions → New repository secret**, and add:

| Name | Value |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | the full contents of `report-bench-updater.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | the contents of `password.txt` |

Keep both files somewhere safe too (for example a password manager). Without them you can never
publish another update, and every installed copy would have to be reinstalled by hand.

### 5. Build the first release
1. Go to the **Actions** tab, then **Release Report Bench desktop → Run workflow**.
2. It takes about 10–15 minutes. When it's done, open **Releases** (right-hand side of the repo).
3. Download **Report Bench_1.0.0_x64-setup.exe**. That is the installer you give people.

> Windows may show **"Windows protected your PC"** the first time, because the installer isn't
> signed with a paid code-signing certificate. Click **More info → Run anyway**. It installs per
> user, so no admin rights are needed. A certificate can be added later if this becomes a problem.

---

## Releasing an update to the app

Only needed when the shell itself changes (not for normal Report Bench changes).

1. On GitHub, edit `src-tauri/tauri.conf.json` and raise `"version"` (e.g. `1.0.0` → `1.0.1`).
2. **Actions → Release Report Bench desktop → Run workflow.**
3. When it finishes, everyone gets the *Update now* prompt the next time they open the app.

The version number must always go up; the app ignores releases that aren't newer than what's installed.

## How the update is kept safe
Each release is signed with your private key. The installed app has the matching public key
built in and refuses to install anything that isn't signed with your key.
