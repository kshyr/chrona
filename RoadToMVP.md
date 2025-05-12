# Chrona: Road to MVP

## MVP Definition

The Minimum Viable Product (MVP) for Chrona will enable users to:

1. Create and manage tasks with status tracking
2. Start time-tracking sessions linked to tasks
3. View and create activity logs
4. Navigate between Dashboard and Planning views
5. See basic time statistics
6. Store all data in TOML files

## Implementation Roadmap

### Phase 1: Widget-Based Page System (2 weeks)

- [x] **Task listing widget** (partially complete)
  - [x] Basic task display
  - [ ] Improve task display with clearer status indicators
  - [ ] Add filtering options (by status, date)
  
- [ ] **Page layout infrastructure**
  - [ ] Define `Page` trait and basic implementations
  - [ ] Create widget containers/blocks
  - [ ] Implement Tab/Shift+Tab navigation between widgets
  - [ ] Add page switching with F1-F4 keys
  
- [ ] **Basic Dashboard page**
  - [ ] Layout with task list, session, and recent logs sections
  - [ ] Active task highlighting
  
- [ ] **Planning page**
  - [ ] Layout with full task list and details panels
  - [ ] Task detail view widget

**Technical tasks:**
- [ ] Create modular widget framework
- [ ] Implement widget state management
- [ ] Design layout management system
- [ ] Set up navigation and focus handling

### Phase 2: Enhanced Task Management (1 week)

- [ ] **Task model improvements**
  - [ ] Add parent/child relationship for subtasks
  - [ ] Implement task grouping
  - [ ] Add task folding mechanism
  
- [ ] **Task visualization enhancements**
  - [ ] Color-coded status indicators
  - [ ] Improved task sorting (by status, date, priority)
  - [ ] Task grouping/folding UI
  
- [ ] **Task actions**
  - [ ] Status transition shortcuts
  - [ ] Bulk actions on selected tasks
  - [ ] Task search functionality

**Technical tasks:**
- [ ] Update task data model
- [ ] Enhance storage to handle hierarchical tasks
- [ ] Implement efficient rendering for folded task groups
- [ ] Extend keyboard shortcuts for task management

### Phase 3: Time Tracking (2 weeks)

- [ ] **Session model**
  - [ ] Design session data structure
  - [ ] Implement session start/pause/resume/stop
  - [ ] Link sessions to tasks
  
- [ ] **Timer widget**
  - [ ] Elapsed time mode
  - [ ] Countdown (Pomodoro) mode
  - [ ] Session controls
  
- [ ] **Session persistence**
  - [ ] Save sessions to TOML
  - [ ] Load and resume interrupted sessions
  
- [ ] **Focus page**
  - [ ] Layout with current task, timer, and session logs
  - [ ] Pomodoro controls and indicators

**Technical tasks:**
- [ ] Create time tracking engine
- [ ] Design session storage schema
- [ ] Implement timer thread with safe state updates
- [ ] Add session serialization/deserialization
- [ ] Build timer UI components

### Phase 4: Activity Logging (1 week)

- [ ] **Log entry model**
  - [ ] Define log entry data structure
  - [ ] Link logs to tasks and sessions
  
- [ ] **Log capture**
  - [ ] Automatic logging of system events
  - [ ] Manual entry interface with timestamp
  
- [ ] **Log visualization**
  - [ ] Chronological log view widget
  - [ ] Filtering by task, session, date
  
- [ ] **Log persistence**
  - [ ] Store logs in TOML
  - [ ] Efficient querying of log history

**Technical tasks:**
- [ ] Create activity log data structures
- [ ] Implement logging subsystem
- [ ] Build log entry UI and input handling
- [ ] Add log storage and retrieval

### Phase 5: Stats and Storage (1 week)

- [ ] **Basic statistics**
  - [ ] Time summary calculations (day/week/month)
  - [ ] Task completion metrics
  
- [ ] **Stats page**
  - [ ] Layout with time summary and task completion widgets
  - [ ] Simple activity timeline visualization
  
- [ ] **Storage improvements**
  - [ ] Optimize TOML storage format
  - [ ] Add input validation and error handling
  - [ ] Implement data backup mechanism

**Technical tasks:**
- [ ] Build statistics calculation engine
- [ ] Create visualization components for stats
- [ ] Enhance TOML storage for efficiency and robustness
- [ ] Add automated backup functionality

### Phase 6: CLI and Polish (1 week)

- [ ] **Command Line Interface**
  - [ ] Task management commands
  - [ ] Session controls
  - [ ] Log entry
  - [ ] Reporting
  
- [ ] **UI Polish**
  - [ ] Consistent styles across all pages
  - [ ] Responsive layouts for different terminal sizes
  - [ ] Enhanced help screens
  
- [ ] **Error handling**
  - [ ] Comprehensive error messages
  - [ ] Recovery mechanisms
  - [ ] Status bar notifications

**Technical tasks:**
- [ ] Create CLI command parser
- [ ] Implement CLI commands using core library
- [ ] Add responsive design for all widgets
- [ ] Enhance error handling throughout the application

## Integration Points

The MVP will focus on these key integration points between components:

1. **Task → Timer Connection**
   - Selecting a task automatically fills context for time tracking
   - Starting a timer updates task status to "In Progress"

2. **Timer → Log Connection**
   - Starting/pausing/stopping a session creates automatic log entries
   - Session duration is captured in logs

3. **Task → Log Connection**
   - Task status changes are automatically logged
   - Task completion updates time reports

4. **Data Persistence**
   - All components save to a consistent TOML format
   - Data is recoverable across application restarts

## Testing Requirements

1. **Unit Tests**
   - Core logic for tasks, sessions, and logs
   - Data model serialization/deserialization

2. **Integration Tests**
   - Page navigation flows
   - Task-timer-log interactions
   - Data persistence and recovery

3. **Manual Testing Scenarios**
   - Create task → start timer → add log → complete task
   - Multi-day sessions with pauses
   - Application crash recovery
   - Different terminal sizes and environments

## MVP Launch Checklist

- [ ] All features from Phases 1-6 implemented
- [ ] Passing tests for core functionality
- [ ] Documentation:
  - [ ] README with installation instructions
  - [ ] User guide with key commands
  - [ ] Configuration options
- [ ] Packaging:
  - [ ] Cargo release configuration
  - [ ] Binary releases for major platforms
- [ ] Initial version published to crates.io

## Post-MVP Priorities

1. Obsidian vault integration
2. Enhanced visualizations and charts
3. Custom workflow configurations
4. Expanded CLI capabilities
5. Theme customization

---

This roadmap will be revisited and updated as development progresses, with tasks being checked off as they're completed.