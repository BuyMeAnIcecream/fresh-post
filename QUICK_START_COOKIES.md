# Quick Start: Testing with Your LinkedIn Account

## Step 1: Export Your Cookies

### Chrome/Edge (Easiest Method):

1. **Log into LinkedIn** in your browser
   - Go to https://www.linkedin.com
   - Make sure you're logged in

2. **Open DevTools**
   - Press `F12` or right-click → "Inspect"
   - Go to the **Application** tab (Chrome) or **Storage** tab (Edge)

3. **Find Cookies**
   - In left sidebar: **Cookies** → `https://www.linkedin.com`

4. **Copy the `li_at` cookie**
   - Find the row with name `li_at`
   - Copy the **Value** column (it's a long string)

5. **Create the cookie file**
   ```bash
   # In the project directory, create:
   echo "li_at=PASTE_YOUR_VALUE_HERE" > linkedin_cookies.txt
   ```
   
   Or manually create `linkedin_cookies.txt` with:
   ```
   li_at=your_long_token_value_here
   ```

### Alternative: Get All Cookies via Console

1. Go to LinkedIn (logged in)
2. Open DevTools (F12) → **Console** tab
3. Run this:
   ```javascript
   document.cookie.split(';').filter(c => c.includes('li_at')).forEach(c => console.log(c.trim()));
   ```
4. Copy the output and create `linkedin_cookies.txt`

## Step 2: Test It!

```bash
cargo run
```

You should see:
```
🍪 Loaded 1 cookies from linkedin_cookies.txt
```

And you should get **more jobs** than in guest mode!

## Step 3: Verify It's Working

- Check the output - you should see more jobs
- Check `debug_linkedin.html` - it should show your logged-in view
- The HTML should be larger (more content)

## Troubleshooting

**"Cookie file not found"**
- Make sure file is named exactly `linkedin_cookies.txt`
- Make sure it's in the project root (same folder as `Cargo.toml`)

**Still getting limited results**
- Make sure `li_at` cookie value is correct (no extra spaces)
- Try logging out and back into LinkedIn, then export cookies again
- Cookies expire - you may need to refresh them

**Getting errors**
- Make sure the cookie format is: `name=value` (no spaces around `=`)

## Security Reminder

⚠️ **NEVER commit `linkedin_cookies.txt` to git!**

The file is already in `.gitignore`, but:
- Your `li_at` cookie = full access to your LinkedIn account
- Treat it like a password
- Delete it when done testing
