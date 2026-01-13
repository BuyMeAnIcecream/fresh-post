// JavaScript snippet to run in LinkedIn DevTools Console
// This will output cookies in the format needed for linkedin_cookies.txt

// Method 1: Get all cookies (recommended)
console.log("=== All LinkedIn Cookies ===");
console.log("Copy everything below this line:");
console.log("---");
document.cookie.split(';').forEach(cookie => {
    const trimmed = cookie.trim();
    if (trimmed) {
        console.log(trimmed);
    }
});
console.log("---");
console.log("\n✅ Copy the lines above and paste them into linkedin_cookies.txt");

// Method 2: Get just the li_at cookie
console.log("\n=== Just li_at Cookie ===");
const liAtCookie = document.cookie.split(';').find(c => c.trim().startsWith('li_at='));
if (liAtCookie) {
    console.log(liAtCookie.trim());
    console.log("\n✅ Copy the line above and paste it into linkedin_cookies.txt");
} else {
    console.log("❌ li_at cookie not found. Make sure you're logged in!");
}
