## ADDED Requirements

### Requirement: Animated directional feedback for wrong guesses
After each incorrect guess, the game SHALL display an animated feedback message indicating whether the guess was too high or too low, using motion cues (arrows or directional ASCII art) and color.

#### Scenario: Too low feedback animation
- **WHEN** the user's guess is lower than the secret number
- **THEN** the message "Too low!" SHALL appear with an upward arrow animation (e.g., `↑↑↑`) and yellow color, with characters appearing progressively

#### Scenario: Too high feedback animation
- **WHEN** the user's guess is higher than the secret number
- **THEN** the message "Too high!" SHALL appear with a downward arrow animation (e.g., `↓↓↓`) and red color, with characters appearing progressively

#### Scenario: Feedback animation completes before next prompt
- **WHEN** the animated feedback is playing
- **THEN** the next guess prompt SHALL NOT appear until the animation finishes

### Requirement: Animated attempt counter
After each guess, the game SHALL display the current attempt number with a brief highlight animation.

#### Scenario: Attempt count updates with animation
- **WHEN** a guess is submitted
- **THEN** the attempt counter line SHALL flash or briefly bold before settling to its normal display state

### Requirement: Animated invalid input warning
When the user enters non-numeric input, the game SHALL display a shaking or flashing warning message.

#### Scenario: Invalid input triggers animated warning
- **WHEN** the user enters text that cannot be parsed as a number
- **THEN** the "Please enter a valid number!" message SHALL appear with a rapid color-flash or repeated character effect (e.g., `!!! Please enter a valid number !!!`)
- **THEN** the prompt SHALL re-appear for another input attempt
