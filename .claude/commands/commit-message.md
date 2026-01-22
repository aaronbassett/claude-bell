---
name: commit-message
description: Validate, fix, or generate conventional commit messages with strict formatting rules
argument-hint: "[check|fix|suggest] [message]"
allowed-tools:
  - Bash
  - Read
---

# Commit Message Command

## Purpose

Validate, auto-fix, or generate commit messages following Conventional Commits specification with Angular convention. Enforce all formatting rules from the useful-commits skill.

## Subcommands

### check

Validate a commit message against all rules and report violations.

**Usage:**
```bash
/commit-message check "feat: Added new feature."
/commit-message check < commit.txt
echo "fix: bug" | /commit-message check
```

**Output:**
- List all rule violations
- Provide specific guidance for each violation
- Show the corrected version

### fix

Automatically fix violations and return the corrected message.

**Usage:**
```bash
/commit-message fix "feat: Added new feature."
/commit-message fix < commit.txt
echo "fix: bug" | /commit-message fix
```

**Output:**
- Corrected commit message
- Explanation of changes made
- Remaining issues (if unfixable automatically)

### suggest

Generate a commit message from staged git changes.

**Usage:**
```bash
/commit-message suggest
```

**Requirements:**
- Must be run in a git repository
- Must have staged changes (git add)

**Output:**
- Generated commit message following all rules
- Brief explanation of changes detected
- Suggestion for body content if changes are complex

## Input Methods

The command accepts messages via:

1. **Argument**: `/commit-message check "message here"`
2. **Stdin**: `echo "message" | /commit-message check`
3. **File**: `/commit-message check < file.txt`

For `suggest` subcommand, no message argument is needed (reads from git diff).

## Implementation Instructions

### Step 1: Parse Subcommand and Input

Determine which subcommand was invoked:
- `check` - validation only
- `fix` - validation + auto-correction
- `suggest` - generate from git diff

For `check` and `fix`, read the commit message from:
- Command argument if provided
- Stdin if no argument

For `suggest`, ignore any message argument and read from git diff.

### Step 2: Validation Rules

Check the message against all rules from the useful-commits skill:

**Subject Line Checks:**
- [ ] Has type prefix (`feat:`, `fix:`, etc.)
- [ ] Type is one of the allowed Angular convention types
- [ ] Description is lowercase (first letter after colon+space)
- [ ] No period at end of subject
- [ ] Subject ≤70 characters (including type and scope)
- [ ] Imperative mood (use patterns: "add" not "added", "fix" not "fixed")
- [ ] Scope format valid if present: `(scope)` with lowercase letters

**Body Checks (if present):**
- [ ] Blank line separates subject from body
- [ ] Lines wrap at 90 characters
- [ ] Body ≤700 characters (excluding footers)
- [ ] ≤8 bullet points if using bullets
- [ ] First letter capitalized in body paragraphs

**Prohibited Content:**
- [ ] No references to `./specs/**/*.md` or local spec IDs
- [ ] No `SPEC-123` or `[#DOC-456]` style references
- [ ] GitHub issue references (#123) are allowed

**Breaking Change Checks:**
- [ ] If "!"" present, `BREAKING CHANGE:` footer should exist
- [ ] If `BREAKING CHANGE:` footer present, "!"" should be in subject
- [ ] Breaking change description is informative

### Step 3: Specific Violation Detection

For each violation, provide specific guidance:

**Capitalization violations:**
```
❌ Subject is capitalized: "feat: Add feature"
✅ Should be: "feat: add feature"
```

**Period violations:**
```
❌ Subject ends with period: "fix: correct bug."
✅ Should be: "fix: correct bug"
```

**Character limit violations:**
```
❌ Subject is 75 characters (max 70)
✅ Consider shortening or using scope: "feat(auth): add JWT token refresh"
```

**Imperative mood violations:**
```
❌ Past tense used: "feat: added feature"
✅ Should be: "feat: add feature"

❌ Present tense used: "fix: fixes bug"
✅ Should be: "fix: fix bug"
```

**Type violations:**
```
❌ Invalid type: "update: change code"
✅ Use one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore
```

### Step 4: Auto-Fix Logic (for fix subcommand)

Apply automatic corrections:

**Fixable automatically:**
1. Capitalize → lowercase first letter after colon+space
2. Remove trailing period from subject
3. Convert past tense to imperative (common patterns):
   - "added" → "add"
   - "fixed" → "fix"
   - "updated" → "update"
   - "removed" → "remove"
   - "implemented" → "implement"
4. Convert present tense to imperative:
   - "adds" → "add"
   - "fixes" → "fix"
   - "updates" → "update"
5. Add missing blank line between subject and body
6. Wrap body lines at 90 characters
7. Lowercase scope if capitalized

**Not fixable automatically (require manual intervention):**
1. Missing type prefix
2. Invalid type
3. Vague descriptions
4. Subject too long (need to rethink phrasing)
5. Body over 700 characters (need to condense)
6. More than 8 bullet points (need to consolidate)
7. Local spec references (need to remove or replace)

For unfixable issues, list them clearly with guidance.

### Step 5: Generate from Git Diff (suggest subcommand)

When generating a commit message:

1. **Check git repository and staged changes:**
   ```bash
   git rev-parse --is-inside-work-tree
   git diff --cached --stat
   ```

2. **Analyze staged changes:**
   ```bash
   git diff --cached --name-status
   git diff --cached
   ```

3. **Determine appropriate type:**
   - New files → `feat:`
   - Test files → `test:`
   - Documentation → `docs:`
   - Package.json/dependencies → `build:`
   - CI config → `ci:`
   - Code changes:
     - Bug fixes → `fix:`
     - New functionality → `feat:`
     - Code cleanup → `refactor:`
     - Performance → `perf:`
     - Style only → `style:`

4. **Identify scope (optional):**
   - Look for common directory or component
   - Only include if clear and consistent with existing commits

5. **Draft subject line:**
   - Imperative mood
   - Lowercase
   - ≤70 characters
   - Specific and distinctive

6. **Draft body (if needed):**
   - Explain what and why
   - Keep under 700 characters
   - Wrap at 90 characters

7. **Check for breaking changes:**
   - API signature changes
   - Removed functionality
   - Behavior changes
   - Configuration changes
   - If breaking, add "!"" and `BREAKING CHANGE:` footer

### Step 6: Output Format

**For check subcommand:**

```
Validating commit message...

❌ Subject is capitalized (should be lowercase after colon)
❌ Subject ends with period (should not)
✅ Type is valid (feat)
✅ Subject length is 45 characters (within 70 limit)
❌ Body exceeds 700 characters (currently 832)

Original message:
---
feat: Added new authentication feature.

This commit adds JWT authentication with refresh tokens and implements
the OAuth2 flow for third-party authentication. The system now supports
multiple authentication providers including Google, GitHub, and Microsoft.
The token refresh mechanism automatically renews tokens before expiry to
prevent user session interruptions. Additionally, the authentication
service has been refactored to use a more modular architecture that
allows for easy addition of new authentication providers in the future.
The implementation includes comprehensive unit tests and integration
tests covering all authentication flows and edge cases. Documentation
has been updated to reflect the new authentication patterns and usage
examples for developers integrating with the authentication system.
---

Corrected message:
---
feat: add JWT authentication with OAuth2 support

Implement JWT authentication with automatic token refresh and OAuth2
flow for third-party providers (Google, GitHub, Microsoft).

Key changes:
- JWT token generation and validation
- Automatic token refresh before expiry
- OAuth2 provider integration
- Modular architecture for adding new providers

This prevents session interruptions and enables enterprise SSO
integration.

---

Suggestions:
- Body is still 312 characters. Consider condensing further if possible.
- Consider splitting into multiple commits if adding too many features.
```

**For fix subcommand:**

```
Fixed commit message:

feat: add JWT authentication with OAuth2 support

Implement JWT authentication with automatic token refresh and OAuth2
flow for third-party providers (Google, GitHub, Microsoft). Modular
architecture enables easy addition of new authentication providers.

Changes made:
✅ Lowercased subject ("Added" → "add")
✅ Removed trailing period
✅ Changed to imperative mood
✅ Condensed body from 832 to 287 characters
✅ Wrapped lines at 90 characters

Ready to use!
```

**For suggest subcommand:**

```
Analyzing staged changes...

Files changed:
  M  src/auth/jwt-service.ts
  M  src/auth/oauth-provider.ts
  A  src/auth/token-refresh.ts
  M  tests/auth/jwt.test.ts

Suggested commit message:
---
feat(auth): add JWT token refresh mechanism

Implement automatic token refresh when tokens expire within 5 minutes
of API calls. This prevents users from being logged out during active
sessions. Refresh happens transparently in the background.

---

Rationale:
- Type: feat (new functionality added)
- Scope: auth (authentication system)
- Focus: JWT token refresh (main feature added)
- Description follows imperative mood, lowercase, no period
- Body explains what, why, and how (under 700 chars)
- Subject is 51 characters (within 70 limit)

You can edit this message or use it as-is.
```

## Error Handling

Handle common errors gracefully:

**Not a git repository (for suggest):**
```
❌ Error: Not in a git repository
   Run this command from within a git repository.
```

**No staged changes (for suggest):**
```
❌ Error: No staged changes found
   Use 'git add' to stage changes before generating a commit message.
```

**Empty message (for check/fix):**
```
❌ Error: No commit message provided
   Provide a message as argument or via stdin.
```

**Invalid subcommand:**
```
❌ Error: Unknown subcommand "validate"
   Valid subcommands: check, fix, suggest
   Usage: /commit-message [check|fix|suggest] [message]
```

## Tips

**For users:**
- Use `check` during code review to validate existing commits
- Use `fix` to quickly correct common formatting mistakes
- Use `suggest` to generate well-formatted messages from changes
- Pipe output to clipboard for easy use: `/commit-message suggest | pbcopy`

**For implementation:**
- Load the useful-commits skill context for comprehensive rule reference
- Use git commands via Bash tool for repository operations
- Apply all formatting rules strictly—this enforces good commit hygiene
- Be specific in error messages—generic "format error" isn't helpful
- Show both the problem and the solution in validation output

## Integration with /commit

This command complements the built-in `/commit` command:
- `/commit` creates actual commits using useful-commits skill guidance
- `/commit-message` validates and fixes messages without committing
- Users can validate before committing: `/commit-message check` then `/commit`

## Examples

**Example 1: Check a message**
```
User: /commit-message check "Fix bug in authentication"

Output:
❌ Missing type prefix
✅ Should be: "fix: prevent null pointer in authentication"
```

**Example 2: Fix a message**
```
User: /commit-message fix "feat: Added new API endpoint."

Output:
Fixed: feat: add new API endpoint

Changes: lowercased "Added" → "add", removed period
```

**Example 3: Generate from git diff**
```
User: /commit-message suggest

Output:
Suggested: feat(api): add user pagination endpoint

[Complete message with body and rationale]
```

**Example 4: Pipe from file**
```
User: /commit-message check < .git/COMMIT_EDITMSG

Output:
[Validation results for commit message from file]
```

## References

Consult the useful-commits skill for:
- Complete list of allowed types
- Detailed formatting rules
- Scope guidelines
- Breaking change notation
- Writing philosophy

Load the skill if needed for comprehensive rule reference:
- `skills/useful-commits/SKILL.md` - Core rules
- `skills/useful-commits/references/` - Detailed guidelines
- `skills/useful-commits/examples/` - Good vs bad examples
