# How to Export LinkedIn Cookies for Testing

This guide will help you export cookies from your browser to test the scraper with your LinkedIn account.

## Method 1: Using Browser DevTools (Recommended)

### Chrome/Edge:

1. **Open LinkedIn and log in**
   - Go to https://www.linkedin.com
   - Make sure you're logged in

2. **Open DevTools**
   - Press `F12` or `Cmd+Option+I` (Mac) / `Ctrl+Shift+I` (Windows/Linux)
   - Go to the **Application** tab (Chrome) or **Storage** tab (Edge)

3. **Find Cookies**
   - In the left sidebar, expand **Cookies**
   - Click on `https://www.linkedin.com`

4. **Copy Important Cookies**
   - Look for these cookies (you may not have all of them):
     - `li_at` (most important - authentication token)
     - `JSESSIONID`
     - `bcookie`
     - `bscookie`
     - `lang`
   
5. **Create cookie file**
   - Create a file named `linkedin_cookies.txt` in the project root
   - Add cookies in this format (one per line):
     ```
     li_at=YOUR_LI_AT_VALUE
     JSESSIONID=YOUR_JSESSIONID_VALUE
     bcookie=YOUR_BCOOKIE_VALUE
     bscookie=YOUR_BSCOOKIE_VALUE
     lang=v=2&lang=en-us
     ```

### Firefox:

1. **Open LinkedIn and log in**
   - Go to https://www.linkedin.com
   - Make sure you're logged in

2. **Open DevTools**
   - Press `F12` or `Cmd+Option+I` (Mac) / `Ctrl+Shift+I` (Windows/Linux)
   - Go to the **Storage** tab

3. **Find Cookies**
   - In the left sidebar, expand **Cookies**
   - Click on `https://www.linkedin.com`

4. **Copy cookies** (same as Chrome above)

## Method 2: Using Browser Extension

### Chrome Extension: "Cookie-Editor" or "EditThisCookie"

1. Install the extension from Chrome Web Store
2. Go to LinkedIn and log in
3. Click the extension icon
4. Export cookies
5. Copy the `li_at` cookie value (and others if needed)
6. Create `linkedin_cookies.txt` with the format above

## Method 3: Manual Copy-Paste (Quick Test)

1. Go to LinkedIn and log in
2. Open DevTools (F12)
3. Go to Console tab
4. Run this JavaScript:
   ```javascript
   document.cookie.split(';').forEach(c => console.log(c.trim()));
   ```
5. Copy the `li_at` cookie line
6. Create `linkedin_cookies.txt` with just that line:
   ```
   li_at=YOUR_VALUE_HERE
   ```

## Cookie File Format

The `linkedin_cookies.txt` file should look like this:

```
li_at=AQEDASx...your_token_here
JSESSIONID="ajax:1234567890"
bcookie="v=2&..."
bscookie="v=2&..."
lang=v=2&lang=en-us
```

**Important Notes:**
- Each cookie should be on its own line
- Format: `name=value` (no spaces around `=`)
- The `li_at` cookie is the most important one
- Lines starting with `#` are ignored (you can add comments)
- Empty lines are ignored

## Security Warning

⚠️ **NEVER commit `linkedin_cookies.txt` to git!**

The cookie file is already in `.gitignore`, but double-check:
- Your cookies give full access to your LinkedIn account
- Treat them like passwords
- Don't share them with anyone
- Delete the file when you're done testing

## Testing

Once you've created `linkedin_cookies.txt`:

```bash
cargo run
```

The scraper will automatically detect and use your cookies if the file exists.

## Troubleshooting

**"Cookie file not found" warning:**
- Make sure the file is named exactly `linkedin_cookies.txt`
- Make sure it's in the project root directory (same folder as `Cargo.toml`)

**Still getting limited results:**
- Check that `li_at` cookie is present and valid
- Try refreshing your LinkedIn session (log out and log back in)
- Cookies expire - you may need to export them again

**Getting blocked:**
- LinkedIn may detect automated requests
- Try adding a delay between requests
- Make sure your user-agent matches your browser
