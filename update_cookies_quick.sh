#!/bin/bash

# Quick script to update just the li_at cookie
# Usage: ./update_cookies_quick.sh

COOKIE_FILE="linkedin_cookies.txt"

echo "🍪 Quick LinkedIn Cookie Update"
echo ""
echo "Instructions:"
echo "1. Go to https://www.linkedin.com (logged in)"
echo "2. Open DevTools (F12 or Cmd+Option+I)"
echo "3. Go to Application tab → Cookies → https://www.linkedin.com"
echo "4. Find the 'li_at' cookie"
echo "5. Copy its Value (the long string)"
echo ""
read -p "Paste the li_at cookie value here: " li_at_value

# Clean up the value (remove quotes, spaces)
li_at_value=$(echo "$li_at_value" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//' | sed 's/^"//' | sed 's/"$//')

if [ -z "$li_at_value" ]; then
    echo "❌ Error: No value provided"
    exit 1
fi

# Save to file
echo "li_at=$li_at_value" > "$COOKIE_FILE"

echo ""
echo "✅ Cookie saved to $COOKIE_FILE"
echo ""
echo "📋 File contents:"
cat "$COOKIE_FILE"
echo ""
