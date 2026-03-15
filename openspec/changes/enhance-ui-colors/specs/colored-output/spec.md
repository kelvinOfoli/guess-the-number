## ADDED Requirements

### Requirement: Colored game header
The game SHALL display a styled ASCII banner in bold cyan when it starts, making the title visually prominent.

#### Scenario: Game start banner
- **WHEN** the game is launched
- **THEN** the header "=== Guess the Number! ===" SHALL be displayed in bold cyan

#### Scenario: Range prompt is styled
- **WHEN** the game is launched
- **THEN** the "I'm thinking of a number between 1 and 100." message SHALL be displayed in cyan

### Requirement: Color-coded guess feedback
The game SHALL use distinct colors for each feedback message to help the player quickly interpret hints.

#### Scenario: Too low feedback
- **WHEN** the player's guess is less than the secret number
- **THEN** the "Too low!" feedback SHALL be displayed in yellow

#### Scenario: Too high feedback
- **WHEN** the player's guess is greater than the secret number
- **THEN** the "Too high!" feedback SHALL be displayed in red

#### Scenario: Correct guess
- **WHEN** the player's guess matches the secret number
- **THEN** the win message SHALL be displayed in bold green
- **THEN** the attempt count SHALL be displayed in bold white

### Requirement: Styled input prompt
The game SHALL display the input prompt in cyan so it is visually distinct from feedback messages.

#### Scenario: Input prompt styling
- **WHEN** the game prompts the player to enter a guess
- **THEN** the "Enter your guess:" prompt SHALL be displayed in cyan

### Requirement: Invalid input warning
The game SHALL display invalid input warnings in bright red to clearly signal an error.

#### Scenario: Non-numeric input
- **WHEN** the player enters a non-numeric value
- **THEN** the "Please enter a valid number!" message SHALL be displayed in bright red

### Requirement: Graceful degradation
The game SHALL disable colors automatically when the output is not a TTY or the NO_COLOR environment variable is set.

#### Scenario: Non-TTY output
- **WHEN** the game output is piped to a file or another process
- **THEN** all output SHALL be plain text without ANSI escape codes
