# Chrona Design Document

## Design Philosophy

Chrona's design is rooted in terminal simplicity and efficiency. The interface puts tasks, time, and logs at your fingertips while maintaining a distraction-free environment. Key principles include:

- **Keyboard-first navigation**: Complete usability without a mouse
- **Focused workflows**: Pages designed around specific activities
- **Visual clarity**: Clear status indicators and minimal visual noise
- **Consistent patterns**: Familiar keyboard shortcuts and navigation across the application

## UI Architecture

### Widget-Based System

The interface is built on a modular widget system:

- **Widgets**: Self-contained UI components for specific functions (task lists, timers, logs)
- **Blocks**: Container elements that position widgets on pages
- **Pages**: Collections of blocks arranged for specific workflows

This architecture allows for flexibility in layout while maintaining a cohesive experience.

## Page Types & Layouts

### 1. Dashboard
The central hub showing your current work context:

```
┌───────────────────────────┐┌───────────────────────────┐
│                           ││                           │
│     ACTIVE TASKS          ││     CURRENT SESSION       │
│                           ││                           │
│ → [*] Implement API       ││  ⏱ 01:24:30              │
│   [ ] Write docs          ││  📌 Implement API         │
│   [ ] Create tests        ││  ▶ Started: 14:05         │
│                           ││                           │
└───────────────────────────┘└───────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│                                                         │
│     RECENT ACTIVITY                                     │
│                                                         │
│  [14:05] Started session on "Implement API"             │
│  [13:50] Completed task "Fix login bug"                 │
│  [11:30] Created task "Write docs"                      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 2. Planning
Comprehensive task management with details:

```
┌───────────────────────────┐┌───────────────────────────┐
│                           ││                           │
│     ALL TASKS             ││     TASK DETAILS          │
│                           ││                           │
│   [ ] Implement API       ││  Title: Implement API     │
│ → [ ] Write docs          ││  Status: Todo             │
│   [ ] Create tests        ││  Created: 2023-09-15      │
│   [*] Fix login bug       ││                           │
│                           ││  Description:             │
│                           ││  Create REST endpoints for │
│                           ││  user authentication and   │
│                           ││  profile management.       │
│                           ││                           │
└───────────────────────────┘└───────────────────────────┘
```

### 3. Focus
Dedicated view for deep work sessions:

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│     CURRENT TASK                                        │
│                                                         │
│     Write docs                                          │
│     Status: In Progress                                 │
│                                                         │
└─────────────────────────────────────────────────────────┘
┌───────────────────────────┐┌───────────────────────────┐
│                           ││                           │
│     TIMER                 ││     SESSION LOGS          │
│                           ││                           │
│  ⏱ 00:45:00 remaining    ││  [15:30] Added section on │
│  🍅 Pomodoro: 2/4         ││        API endpoints      │
│  ▶ Started: 15:00         ││  [15:15] Outlined user    │
│  ⏸ Pause | ⏹ Stop         ││        authentication docs│
│                           ││  [15:00] Started session  │
│                           ││                           │
└───────────────────────────┘└───────────────────────────┘
```

### 4. Stats
Visualization of productivity data:

```
┌───────────────────────────┐┌───────────────────────────┐
│                           ││                           │
│     TIME SUMMARY          ││     TASK COMPLETION       │
│                           ││                           │
│  Today: 4h 30m            ││  Completed this week: 7   │
│  Week:  22h 15m           ││  In progress: 3           │
│  Month: 87h 45m           ││  Backlog: 12              │
│                           ││                           │
└───────────────────────────┘└───────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│                                                         │
│     ACTIVITY TIMELINE                                   │
│                                                         │
│  Mon ████████████████░░░░░░░░░░░░░░░░░░░░░░░░ 4h 30m   │
│  Tue █████████████████████░░░░░░░░░░░░░░░░░░░ 5h 45m   │
│  Wed ██████████████████████████░░░░░░░░░░░░░░ 6h 30m   │
│  Thu █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3h 15m   │
│  Fri ██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2h 15m   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Core UI Components

### Task List
- Color-coded status indicators
- Checkbox selection for actions
- Hierarchical display with folding for task groups
- Clear status transitions (Backlog → Todo → In Progress → Done)

### Timer Widget
- Two modes: Elapsed time and Countdown (Pomodoro)
- Visual session status (active, paused, completed)
- Task association display
- Quick actions for session control

### Activity Log
- Chronological entries with timestamps
- Automatic and manual entries
- Contextual relationships to tasks and sessions
- Filtering options

## Navigation & Interaction

### Global Keyboard Shortcuts
- `?`: Toggle help screen
- `q`, `Esc`: Exit application or cancel current action
- `Tab`/`Shift+Tab` or `[0-9]`: Navigate between widgets
- `Shift+[0-9]`: Switch between pages

### Task Navigation
- `j`/`k` or arrow keys: Move selection
- `Space`: Toggle picked task(s)
- `a`: Add new task
- `e`: Edit selected task
- `d`: Delete selected task
- `/`: Search tasks

### Time Tracking
- `t`: Start timer on selected tasks
- `p`: Pause/resume current session
- `s`: Stop current session
- `m`: Switch timer mode (elapsed/remaining)

### Form Controls
- `Tab`/`Shift+Tab`: Navigate between fields
- `Enter`: Submit/confirm
- `Esc`: Cancel (close dialog)

## Visual Design

### Color System
- **Status colors**:
  - Backlog: Gray
  - Todo: Blue
  - In Progress: Yellow
  - Done: Green
- **Accent colors**:
  - Primary action: Cyan
  - Warning/important: Red
  - Selection: Yellow on dark gray

### Typography & Layout
- Clear hierarchical structure with consistent spacing
- Judicious use of borders to delineate sections
- Bold text for important elements, dim text for secondary information

## Interaction Patterns

### Task Management Flow
1. Create task (default status: Backlog)
2. Move to Todo when planned
3. Start timer to automatically move to In Progress
4. Complete task to move to Done and log completion

### Session Flow
1. Select task(s) to work on
2. Start session (optional timer mode selection)
3. Automatically create session start log
4. Pause/resume as needed
5. End session to capture time and create completion log

### Log Entry Flow
1. Automatic entries from system events
2. Manual entries with timestamps [HH:MM]
3. All entries linked to current task and/or session
4. Searchable history for review

## Accessibility Considerations

- High contrast mode option
- Configurable colors for color-blindness accommodation
- Complete keyboard control without mouse dependency
- Help screen accessible at any point with `?`

## Future Design Directions

- User-configurable widget layouts
- Light/dark theme toggle
- Compact mode for smaller terminals
- Enhanced data visualizations
- Custom status workflows

---

This design document is a living specification that will evolve alongside the application.
