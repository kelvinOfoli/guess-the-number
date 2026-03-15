## ADDED Requirements

### Requirement: Victory animation on correct guess
The game SHALL display a multi-frame celebration animation when the player guesses the correct number.

#### Scenario: Celebration plays on correct guess
- **WHEN** the user's guess matches the secret number
- **THEN** the game SHALL display an animated celebration sequence (e.g., expanding ASCII star burst, bouncing "YOU WIN!" text, or confetti-style characters filling the screen)
- **THEN** the celebration animation SHALL run for at least 1.5 seconds before showing the final result line

#### Scenario: Final result displays after celebration
- **WHEN** the victory animation completes
- **THEN** the game SHALL display the final message "Correct! You got it in N attempt(s)!" in bold green

### Requirement: Attempt-based celebration intensity
The celebration animation SHALL reflect how efficiently the player solved the puzzle.

#### Scenario: Excellent performance triggers enhanced celebration
- **WHEN** the player guesses correctly in 5 or fewer attempts
- **THEN** the celebration animation SHALL be more elaborate (larger ASCII art, more frames, or additional color cycling)

#### Scenario: Standard performance triggers standard celebration
- **WHEN** the player guesses correctly in 6 or more attempts
- **THEN** the game SHALL display the standard celebration animation

### Requirement: Win message types in after celebration
The final win message SHALL be revealed using the same typing effect as the startup animation.

#### Scenario: Win message reveals character by character
- **WHEN** the victory animation finishes
- **THEN** the "Correct! You got it in N attempt(s)!" message SHALL type in character by character at 30-50ms per character
