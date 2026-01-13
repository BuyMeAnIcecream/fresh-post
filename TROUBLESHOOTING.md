# Troubleshooting Guide

## Error: Status 999 (LinkedIn Anti-Bot Protection)

If you see this error:
```
LinkedIn blocked the request (status 999 - anti-bot protection).
```

This means LinkedIn detected automated access and blocked your request. Here's how to fix it:

### Immediate Solutions

1. **Wait and Retry**
   - Wait 5-10 minutes before trying again
   - LinkedIn may temporarily block rapid requests

2. **Refresh Your Cookies**
   - Your cookies may have expired or been flagged
   - Export fresh cookies from your browser:
     1. Go to LinkedIn (logged in)
     2. Open DevTools (F12) → Application → Cookies
     3. Copy the `li_at` cookie value
     4. Update `linkedin_cookies.txt` with the new value

3. **Check Your Cookies File**
   - Make sure `linkedin_cookies.txt` exists and has valid cookies
   - Format: `name=value` (one per line)
   - The `li_at` cookie is the most important one

4. **Use a Different Network**
   - Try from a different network (home vs office, or use VPN)
   - Your IP may be temporarily flagged

5. **Reduce Request Frequency**
   - The scraper already waits 3 seconds before each request
   - If you're running it multiple times quickly, wait longer between runs

### Long-term Solutions

- **Don't run the scraper too frequently** - Once every few hours is reasonable
- **Keep cookies fresh** - Export new cookies every few days
- **Use authenticated requests** - Always use cookies (not guest mode)
- **Be patient** - LinkedIn's anti-bot protection is aggressive

### Verifying Your Setup

1. **Check cookies are loaded:**
   ```
   🍪 Loaded 5 cookies from linkedin_cookies.txt
   ```
   If you don't see this, your cookie file isn't being read.

2. **Check the URL being fetched:**
   ```
   🌐 Fetching: https://www.linkedin.com/jobs/search?...
   ```
   Make sure it looks correct.

3. **Inspect debug HTML:**
   - Check `debug_linkedin.html` after a run
   - If it's very small or contains error messages, LinkedIn blocked you
   - If it's large (1MB+), you got the full page

## Other Common Issues

### "Cookie file not found"
- Make sure `linkedin_cookies.txt` is in the project root
- Check the file name is exactly `linkedin_cookies.txt`

### "No jobs found"
- Check your search parameters in `config.toml`
- Try a broader search (fewer keywords, larger location)
- Some searches legitimately return no results

### "Jobs showing as Unknown Company"
- This is a parsing issue with LinkedIn's JSON structure
- The scraper is working, but LinkedIn's format may have changed
- Check `debug_linkedin.html` to see the raw data
