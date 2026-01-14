#!/bin/bash

# Script to update linkedin_cookies.txt
# This script helps you easily update your LinkedIn cookies

DATA_DIR="${DATA_DIR:-.}"
COOKIE_FILE="$DATA_DIR/linkedin_cookies.txt"

echo "🍪 LinkedIn Cookie Updater"
echo "=========================="
echo ""
echo "This script will help you update your LinkedIn cookies."
echo ""
echo "Method 1: Paste cookies from browser DevTools (Recommended)"
echo "  1. Go to https://www.linkedin.com (make sure you're logged in)"
echo "  2. Open DevTools (F12 or Cmd+Option+I)"
echo "  3. Go to Console tab"
echo "  4. Run: document.cookie"
echo "  5. Copy the output"
echo ""
echo "Method 2: Get just the li_at cookie"
echo "  1. Go to https://www.linkedin.com (logged in)"
echo "  2. Open DevTools (F12) → Application → Cookies"
echo "  3. Find 'li_at' cookie and copy its value"
echo "  4. Paste it when prompted"
echo ""
read -p "Press Enter to continue..."

echo ""
echo "Choose an option:"
echo "  1) Paste full cookie string (from document.cookie)"
echo "  2) Paste just the li_at cookie value"
echo "  3) Manual entry (one cookie per line)"
read -p "Enter choice [1-3]: " choice

case $choice in
    1)
        echo ""
        echo "Paste the full cookie string from 'document.cookie' and press Enter:"
        echo "(Then press Ctrl+D or type 'done' on a new line to finish)"
        echo ""
        read -r cookie_string
        
        # Convert cookie string to format: name=value (one per line)
        echo "$cookie_string" | tr ';' '\n' | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//' | grep -v '^$' > "$COOKIE_FILE"
        
        echo ""
        echo "✅ Cookies saved to $COOKIE_FILE"
        ;;
    2)
        echo ""
        read -p "Paste the li_at cookie value: " li_at_value
        
        # Remove any quotes or extra spaces
        li_at_value=$(echo "$li_at_value" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//' | sed 's/^"//' | sed 's/"$//')
        
        echo "li_at=$li_at_value" > "$COOKIE_FILE"
        
        echo ""
        echo "✅ li_at cookie saved to $COOKIE_FILE"
        ;;
    3)
        echo ""
        echo "Enter cookies one per line (format: name=value)"
        echo "Press Enter on an empty line when done"
        echo ""
        
        > "$COOKIE_FILE"  # Clear the file
        
        while true; do
            read -p "Cookie (or press Enter to finish): " cookie_line
            if [ -z "$cookie_line" ]; then
                break
            fi
            echo "$cookie_line" >> "$COOKIE_FILE"
        done
        
        echo ""
        echo "✅ Cookies saved to $COOKIE_FILE"
        ;;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

echo ""
echo "📋 Current cookies in $COOKIE_FILE:"
echo "-----------------------------------"
cat "$COOKIE_FILE"
echo "-----------------------------------"
echo ""
echo "💡 Tip: The most important cookie is 'li_at' - make sure it's present!"
echo ""
