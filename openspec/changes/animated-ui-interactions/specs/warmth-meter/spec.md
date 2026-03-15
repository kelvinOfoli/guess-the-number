## ADDED Requirements

### Requirement: Proximity warmth bar displayed after each guess
After each incorrect guess, the game SHALL display an animated warmth bar showing how close the current guess is to the secret number.

#### Scenario: Warmth bar appears after first guess
- **WHEN** the first incorrect guess is submitted
- **THEN** a warmth bar SHALL appear below the feedback message showing the proximity level as a filled/empty bar (e.g., `Warmth: [████░░░░░░]`)

#### Scenario: Warmth bar fills left to right on reveal
- **WHEN** the warmth bar is first displayed for a guess
- **THEN** the bar SHALL animate by filling from left to right, one block at a time, with a short delay between blocks

### Requirement: Warmth bar updates in place on subsequent guesses
After the first guess, the warmth bar SHALL update in place (overwriting the previous bar) rather than appending a new line.

#### Scenario: Bar overwrites previous bar
- **WHEN** a second or later incorrect guess is submitted
- **THEN** the warmth bar from the previous guess SHALL be replaced in place by the new bar value
- **THEN** the new bar SHALL animate its fill even when updating

### Requirement: Warmth levels reflect proximity to secret number
The warmth bar fill level SHALL reflect how close the guess is to the secret number on an abstract scale without revealing the exact distance.

#### Scenario: Close guess shows high warmth
- **WHEN** the guess is within 10 of the secret number
- **THEN** the warmth bar SHALL be at least 70% filled

#### Scenario: Far guess shows low warmth
- **WHEN** the guess is more than 40 away from the secret number
- **THEN** the warmth bar SHALL be at most 30% filled

#### Scenario: Warmth bar color reflects temperature
- **WHEN** warmth is low (≤ 30%)
- **THEN** the bar SHALL display in blue or cyan color
- **WHEN** warmth is medium (31%–69%)
- **THEN** the bar SHALL display in yellow color
- **WHEN** warmth is high (≥ 70%)
- **THEN** the bar SHALL display in red or bright red color
