import re
from playwright.sync_api import sync_playwright, Page, expect

def run(playwright):
    """
    This test verifies that a user can log in, navigate to the
    entry list page, and view the entries.
    """
    browser = playwright.chromium.launch(headless=True)
    page = browser.new_page()

    try:
        # 1. Arrange: Go to the login page.
        page.goto("http://localhost:5173/app/")

        # 2. Act: Log in.
        password_input = page.get_by_label("Password")
        expect(password_input).to_be_visible(timeout=10000)
        password_input.fill("debug")

        login_button = page.get_by_role("button", name="Login")
        login_button.click()

        # 3. Assert: Wait for successful login and navigation to the form page.
        expect(page).to_have_url(re.compile(r".*/form"), timeout=10000)
        expect(page.get_by_text("Wie geht's dir?")).to_be_visible()

        # 4. Act: Navigate to the entry list page.
        menu_button = page.locator(".menu-button")
        expect(menu_button).to_be_visible()
        menu_button.click()

        # 5. Assert: Wait for navigation to the entries page.
        expect(page).to_have_url(re.compile(r".*/entries"))
        expect(page.get_by_role("heading", name="Entries")).to_be_visible()

        # 6. Screenshot: Capture the final result for visual verification.
        page.screenshot(path="jules-scratch/verification/entry_list_page.png")
        print("Screenshot saved to jules-scratch/verification/entry_list_page.png")

    finally:
        browser.close()

with sync_playwright() as playwright:
    run(playwright)