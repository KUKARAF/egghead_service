# Frontend Design Brief: Violentmonkey Userscript Platform

## Overview
Design a clean, minimalist platform for users to discover, browse, and create custom violentmonkey userscripts. The platform combines a curated script marketplace with AI-powered custom script generation, allowing users to request personalized automations at transparent, estimated costs.

## Design Language

### Typography
Use the same font family as https://static.osmosis.page/password/ - a clean, modern typeface that conveys simplicity and trustworthiness. The design should feel minimal and focused.

### Color Palette
Base all visual design on the following warm and soft pastel color palette:

| Color Name | HEX | Usage |
|---|---|---|
| Soft Peach | #f5d5c8 | Primary brand color, headers, dominant elements |
| Pastel Blush | #efb7b0 | Accent color, CTAs, highlights, active states |
| Soft Sage | #d8e8d8 | Success states, positive indicators, secondary accents |
| Soft Lavender | #e8d8e8 | Info states, calm messaging, tertiary accents |
| Soft Dusty Blue | #d8e8f0 | Secondary backgrounds, supporting elements |
| Cream | #f8f5e8 | Light backgrounds, card backgrounds |
| Off-White | #f9f9f9 | Default background, neutral spaces |

## Core Features & User Flows

### 1. Authentication & Dashboard
**Primary Screen: User Dashboard**
- Clean header with user profile/account menu
- Display current account balance (in cents, formatted as currency)
- Quick stats: Total scripts created, scripts approved, scripts in progress
- Navigation to main sections: Browse Scripts, Create Script, My Scripts

### 2. Script Browser
**Feature: Discover & Search Existing Scripts**
- Hero section showcasing featured scripts
- Search/filter functionality by:
  - Category (website, productivity, etc.)
  - Popularity
  - Rating
- Script card design showing:
  - Script name
  - Brief description
  - Usage count
  - Rating/reviews
  - Match pattern (websites it works on)
  - Installation CTA button
- Infinite scroll or pagination for browsing

### 3. Create Custom Script Workflow
**Primary Flow: User submits request for custom userscript**

#### Step 1: Script Request Form
- **Input Field**: Prompt textarea where user describes what they want the script to do
  - Placeholder: "Describe what you want the script to automate. Be specific about the websites and actions..."
  - Character counter (suggest 100-500 chars for best results)
- **Optional Input**: Target URL/website field
- **Optional Input**: Upload capability for page HTML or recording (UX TBD - capture what user wants automated)
- **Submit Button**: "Get Cost Estimate" (primary CTA)

#### Step 2: Cost Estimation & Approval
- **Display Section**: Show estimated cost in cents with clear breakdown
  - Estimated price
  - Price rationale explanation (why it costs what it does)
  - Current account balance
  - Warning if insufficient balance
- **Action Buttons**:
  - "Approve & Generate Script" (primary - only enabled if user has sufficient balance)
  - "Edit Request" (secondary)
  - "Cancel" (tertiary)
- **Info Box**: What to expect - "Your script will be generated and available within X minutes"

#### Step 3: Processing State
- **Status Display**: Show task is in progress
  - Status: "Generating your script..."
  - Timestamp of submission
  - Estimated time remaining
  - Option to cancel request (if applicable)
- **Progress Indicator**: Simple visual indicator (spinner, progress bar, or minimal animation)

#### Step 4: Completion States

**Success State:**
- Display generated script details:
  - Script name
  - Full script code (in syntax-highlighted code block)
  - Match pattern (which websites it works on)
  - Copy-to-clipboard button for script code
  - Install button / Export button
- Actions:
  - "View in My Scripts"
  - "Create Another"
  - "Copy Code"

**Error State:**
- Display error message clearly
- Show what went wrong (error_message field)
- Action buttons:
  - "Try Again"
  - "Edit Request"
  - "Contact Support"

### 4. My Scripts
**Feature: View & Manage All Created Scripts**
- Table or card view of user's scripts showing:
  - Script name
  - Status (pending, approved, rejected, error)
  - Cost paid (in cents/currency)
  - Date created
  - Quick actions: View Code, Copy, Delete
- Filtering options:
  - By status
  - By date
  - By cost range
- Sorting options:
  - Newest first
  - Most recent activity
  - Most expensive

**Script Detail View:**
- Full script information
- Code display with syntax highlighting
- Match pattern details
- Installation instructions
- Option to create similar script / request modifications

### 5. Account & Settings
**Feature: User Account Management**
- Profile information display
- Balance display and history
- Payment/top-up option (UX TBD based on payment system)
- Usage statistics
- Activity log of script requests and generations

## Visual Hierarchy & Layout

### Header/Navigation
- Minimal, clean header with service logo/name
- User profile icon/menu in top right
- Navigation menu (Browse, Create, My Scripts, Account)
- Current balance display

### Spacing & Proportions
- Use generous whitespace to match minimalist aesthetic
- Keep content width reasonable for readability
- Responsive design that works on tablet and mobile

### Interactive Elements
- Buttons should use Pastel Blush (#efb7b0) for primary actions
- Hover/active states should darken slightly (add opacity or mix with a darker tone)
- Form inputs with Soft Peach (#f5d5c8) or Soft Dusty Blue (#d8e8f0) borders
- Use Soft Lavender (#e8d8e8) for disabled or secondary states
- Success indicators use Soft Sage (#d8e8d8)
- Ensure sufficient contrast for accessibility (test text color pairings - may need dark text on pastel backgrounds)

### Cards & Containers
- Use subtle borders in Copper Penny or English Lavender
- Light background in Grullo for card backgrounds (or white with colored border)
- Consistent padding and border radius throughout

## Tone & Messaging

- **Professional yet approachable**: This is automation/developer-focused but should feel welcoming
- **Transparent about costs**: Always show pricing clearly, explain what affects cost
- **Clear action labels**: Use specific verbs (Get Cost Estimate, Approve & Generate, not just Submit)
- **Helpful prompts**: Provide good examples and guidance for what makes a good script request
- **Status clarity**: Always tell users what state they're in and what happens next

## Key User Journeys

### Journey 1: Browse & Learn
1. Land on dashboard
2. Browse existing scripts
3. Read script details
4. Understand how platform works

### Journey 2: Create Custom Script
1. Click "Create Script"
2. Write detailed description
3. Get cost estimate
4. Review price and account balance
5. Approve generation
6. Wait for completion
7. Review generated script
8. Use or iterate

### Journey 3: Manage Scripts
1. View "My Scripts"
2. See all past requests and generated scripts
3. View script code
4. Copy or export for use

## Accessibility & Responsive Design
- Ensure all interactive elements are keyboard accessible
- Proper color contrast ratios (WCAG AA minimum)
- Mobile-responsive layout that works on phones, tablets, and desktop
- Clear focus states for keyboard navigation
- Alt text for any icons or images

## Loading & Empty States
- **Empty state (new user)**: Show welcome message with getting started guide
- **Empty My Scripts**: Prompt to create first script
- **Loading states**: Use subtle progress indicators, avoid blocking interactions where possible
- **Error states**: Clear, actionable error messages with recovery paths

## Performance Considerations
- Keep page load fast and focused
- Lazy load script browser content
- Smooth transitions and interactions

---

## Implementation Notes for Designer
Focus on creating a design that feels:
- **Minimal and clean** - inspired by the reference site
- **Warm and approachable** - earth tone palette creates welcoming feel
- **Transparent** - pricing, status, and next steps always visible
- **Focused** - guide users clearly through each workflow without unnecessary distractions
