from playwright.sync_api import sync_playwright, expect

def run_verification():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        try:
            # 1. Navigate to the login page
            page.goto("http://localhost:5173/login")

            # 2. Log in
            password_input = page.get_by_label("Password")
            expect(password_input).to_be_visible()
            password_input.fill("abc")

            login_button = page.get_by_role("button", name="Login")
            expect(login_button).to_be_visible()
            login_button.click()

            # Wait for navigation to the form page after login
            expect(page).to_have_url("http://localhost:5173/form")

            # 3. Navigate to the entry list page
            # The UI has a 'Show Entries' link/button
            show_entries_button = page.get_by_role("button", name="Show Entries")
            expect(show_entries_button).to_be_visible()
            show_entries_button.click()

            # Wait for navigation to the entry list page
            expect(page).to_have_url("http://localhost:5173/entries")

            # 4. Verify the export icon is visible
            export_button = page.locator('.header-actions button').nth(0) # The first button in header-actions
            expect(export_button).to_be_visible()

            # 5. Take a screenshot
            page.screenshot(path="jules-scratch/verification/verification.png")

            print("Screenshot taken successfully.")

        except Exception as e:
            print(f"An error occurred: {e}")
            page.screenshot(path="jules-scratch/verification/error.png")

        finally:
            browser.close()

if __name__ == "__main__":
    run_verification()