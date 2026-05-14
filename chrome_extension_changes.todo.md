# Chrome Extension Changes for egghead_service

## Summary
Malpa extension integration with egghead_service backend. Users can switch from OpenRouter (direct API key) to egghead (server-based with pricing approval).

---

## background.js

- [ ] Add to DEFAULT_SETTINGS:
  ```javascript
  eggheadEnabled: false,
  eggheadApiToken: "",
  eggheadServiceUrl: "https://egghead.osmosis.page",
  recordingEnabled: false,
  ```

- [ ] Add `handleGenerateViaEgghead({prompt, tabId, tabUrl, actionRecording})` function:
  - Gets full HTML from content.js (call getDOM to get both `summary` and `html` fields)
  - Validates eggheadApiToken is set
  - POST /api/tasks with Bearer token; include `action_recording` JSON if present
  - Returns task ID and status
  - Poll /api/tasks/:id every 5s up to 120 attempts
  - On `awaiting_approval`: broadcast "Price: $X.XX — open dashboard to approve"; continue polling
  - On `processing`: broadcast "Generating script…"
  - On `done`: prepend ViolentMonkey header; save script locally (same as OpenRouter); register it; broadcast done
  - On `failed`/`rejected`: broadcast error message and exit

- [ ] In chrome.runtime.onMessage listener, dispatch:
  ```javascript
  if (action === "generate") {
    const handler = settings.eggheadEnabled ? handleGenerateViaEgghead : handleGenerate;
    handler({prompt, tabId, tabUrl, actionRecording}).catch(e => broadcastError(e.message));
    sendResponse({ok: true});
    return false;
  }
  ```

---

## content.js

- [ ] Update `getDOM` message handler:
  - Return an object with BOTH fields:
    ```javascript
    sendResponse({
      summary: buildDOMSummary(message.truncationBytes),
      html: truncateHTML(document.documentElement.outerHTML, message.truncationBytes)
    });
    ```
  - Add `truncateHTML()` helper: slice to truncationBytes, ensure no dangling tags

---

## popup.html + popup.js

**Recording State Machine (only if eggheadEnabled):**
- States: `idle` → `recording` → `recorded` → back to `idle` on send

- [ ] Add "Record" button (visible only when eggheadEnabled)
  - Disabled while idle/recorded
  - Text changes based on state: "Record" (idle) → "Stop" (recording)
  - Click: start recording, inject content script listener

- [ ] While recording active, show event count badge (e.g., "5 events")

- [ ] Content script listener (injected during recording):
  ```javascript
  window.addEventListener('egghead_record_event', (e) => {
    chrome.runtime.sendMessage({action: 'recordEvent', event: e.detail});
  });
  ```

- [ ] Popup message handler for `recordEvent`: append to popup's recording buffer

- [ ] "Stop Recording" button: stop listening, finalize recording JSON

- [ ] "Generate Script" button: send with `actionRecording` field populated from buffer

- [ ] Optional: replay UI showing recorded events (low priority)

---

## settings.html + settings.js

- [ ] Add checkbox: "Use egghead service (paid)" → `eggheadEnabled`
  - Label: "Switch from OpenRouter to server-based AI generation"

- [ ] Add text input: "egghead API Token" → `eggheadApiToken`
  - Type: password (masked)
  - Placeholder: "egghead_..."

- [ ] Add link: "Get your API token" → opens `{eggheadServiceUrl}/settings` in new tab

- [ ] Show/hide egghead section (checkbox + token input) based on eggheadEnabled state

- [ ] Save both fields to chrome.storage.sync on change

---

## popup.html Changes (minimal)

- [ ] Add "Record" button (hidden if not eggheadEnabled):
  ```html
  <button id="recordBtn" style="display: none;">Record</button>
  ```

- [ ] Show/hide in popup.js based on settings.eggheadEnabled

---

## Key Behavioral Notes

1. **Fallback:** If eggheadEnabled = false or token not set, use existing OpenRouter flow
2. **Price display:** On `awaiting_approval`, show `Price: $X.XX` from estimated_price_cents
3. **Recording optional:** User can submit without recording (recording_field = null on backend)
4. **No popup approval:** Extension does NOT approve price. Only web dashboard can approve.
5. **ViolentMonkey header:** Backend returns script_code WITHOUT header when done (extension adds it)
   - Wait, backend adds header in GET response. Extension receives complete script ready to save.
6. **Error handling:** Network errors, API errors, parse errors all broadcast and don't save script

---

## Testing Checklist (Manual)

- [ ] Enable egghead in settings, paste valid token
- [ ] Click "Generate" on a webpage, see task created and "awaiting_approval" message
- [ ] Open dashboard, see price estimate + rationale, click Approve
- [ ] Extension continues polling, shows "Generating script…"
- [ ] See "Done! Script saved." when complete
- [ ] Script in dashboard shows with ==UserScript== header
- [ ] Click "Copy Script" in dashboard, paste into another editor, verify header is present
- [ ] Test with recording: click Record, interact with page, click Stop, click Generate
- [ ] Verify action_recording JSON is reasonable in backend logs

---

## Files Modified

- `background.js` — add handleGenerateViaEgghead, update router
- `content.js` — extend getDOM to return full HTML
- `popup.html` / `popup.js` — add Record button + recording logic
- `settings.html` / `settings.js` — add egghead section
