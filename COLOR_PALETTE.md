# Violentmonkey Userscript Platform - Color Palette

Generated using pastel CLI with warm, soft pastel tones.

## Primary Colors

| Color Name | HEX | RGB | Usage |
|---|---|---|---|
| Soft Peach | #f5d5c8 | rgb(245, 213, 200) | Primary brand color, headers, dominant elements |
| Pastel Blush | #efb7b0 | rgb(239, 183, 176) | CTA buttons, highlights, interactive states |

## Secondary Colors

| Color Name | HEX | RGB | Usage |
|---|---|---|---|
| Soft Sage | #d8e8d8 | rgb(216, 232, 216) | Success states, positive indicators |
| Soft Lavender | #e8d8e8 | rgb(232, 216, 232) | Info states, calm messaging, disabled elements |
| Soft Dusty Blue | #d8e8f0 | rgb(216, 232, 240) | Secondary backgrounds, supporting elements |

## Neutral & Background Colors

| Color Name | HEX | RGB | Usage |
|---|---|---|---|
| Cream | #f8f5e8 | rgb(248, 245, 232) | Light backgrounds, card backgrounds |
| Off-White | #f9f9f9 | rgb(249, 249, 249) | Default background, neutral spaces |
| Dark Text | #2d2d2d | rgb(45, 45, 45) | Primary text (for contrast) |

## Palette Characteristics

- **Warmth**: All colors maintain a warm undertone, creating a cohesive, approachable feel
- **Softness**: High lightness values create a gentle, non-jarring aesthetic
- **Approachability**: Pastel tones feel welcoming and friendly
- **Minimalism**: Clean palette that supports the minimalist design language
- **Accessibility**: Ensure sufficient contrast between text and background colors

## Implementation Notes

### Button States
- **Default**: Pastel Blush (#efb7b0)
- **Hover**: Slightly darker Pastel Blush (reduce lightness by ~10%)
- **Disabled**: Soft Lavender (#e8d8e8)
- **Active/Focus**: Slightly darker, with additional visual indicator

### Status Indicators
- **Success**: Soft Sage (#d8e8d8)
- **Info**: Soft Lavender (#e8d8e8)
- **Warning**: Soft Peach (#f5d5c8) or Pastel Blush (#efb7b0)
- **Error**: Deeper tone of Pastel Blush for visibility

### Text Color
- Primary text: #2d2d2d (dark gray/charcoal) on Cream/Off-White backgrounds
- Secondary text: Darker version of Soft Lavender or dark gray
- Links: Pastel Blush with underline for clarity

### Cards & Containers
- Background: Cream (#f8f5e8) or Off-White (#f9f9f9)
- Border (optional): Soft Peach (#f5d5c8) at reduced opacity

## Color Generation Details

Generated using: `pastel` CLI tool
- Method: Soft desaturation and lightening of vibrant colors
- Palette Type: Warm pastel (complementary to minimalist design aesthetic)
