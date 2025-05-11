# Chrona: Where We're Going

## The Big Idea

Chrona is a simple terminal tool for tracking both your tasks and your time. It's for people who love working in the terminal and want to see patterns in how they spend their time.

## What We Have Now

- Basic task lists (create, edit, delete)
- Task statuses (Backlog, Todo, In Progress, Done)
- Split-screen view for tasks
- Simple form for entering task details
- Help screen with keyboard shortcuts
- Data saved in TOML files

## What's Next

### The Three Core Pieces

1. **Task Management**
   - Cleaner task display showing title and status clearly
   - Color-coded status indicators (Backlog, Todo, In Progress, Done)
   - Tasks that can have sub-tasks
   - Simple folding of task groups
   - Better sorting and filtering

2. **Time Tracking**
   - Start a work session and pick which tasks you're focusing on
   - Two tracking modes: elapsed time and remaining time (pomodoro-style)
   - Track time even without specific tasks
   - See how long you've spent on different tasks
   - Pause and resume tracking

3. **Activity Logging**
   - Chronological record of work activities
   - Automatic logging of sessions and task changes
   - Manual entries with timestamps [HH:MM]
   - Links logs to your current session or tasks
   - Searchable history for pattern analysis

### Widget-Based UI

- Pages with customizable widgets (2-3 per page)
- Each widget focused on a specific function (tasks, time, logging)
- Consistent keyboard shortcuts across the application
- Tab/Shift+Tab navigation between widgets
- Multiple page layouts for different workflows:
  - Dashboard: Active tasks, current session, recent logs
  - Planning: Full task list with details
  - Focus: Active task with timer and logs
  - Stats: Time summaries and productivity metrics

### Storage Options

- Keep the current TOML files (simple and readable)
- Add Obsidian vault integration (store as markdown files)
- Maybe add more options later (database, cloud)

## Building It Step by Step (MVP Definition)

1. **First:** Build the widget-based page system
   - Create flexible layouts with widgets
   - Set up Tab/Shift+Tab navigation between widgets
   - Implement basic Dashboard and Planning pages

2. **Next:** Enhance task management
   - Color-coded status indicators
   - Task grouping and folding
   - Improved filtering and sorting

3. **Then:** Add time tracking
   - Session timer with elapsed/remaining modes
   - Task picking interface
   - Basic stats for today/week/month

4. **Next:** Add activity logging
   - Automatic session and task event logging
   - Manual entry with timestamps [HH:MM]
   - Simple search and filtering

5. **Finally:** Refine storage and add analysis
   - Clean up TOML storage
   - Add basic charts and stats page
   - Prepare for Obsidian integration

### Definition of Ready (MVP)
The MVP is ready when a user can:
- Create and manage tasks with status tracking
- Start time-tracking sessions linked to tasks
- View automatic and create manual activity logs
- Navigate between Dashboard and Planning views
- See basic time statistics for motivation
- Have all data properly saved in TOML files

## Technical Notes

- Keep shared state (like tasks) separate from view state (like selection)
- Make tasks link to sessions and activity logs
- Use efficient rendering for complex displays
- Implement seamless widget integration:
  1. Selecting a task → automatically fills context for time tracking
  2. Starting a session → creates automatic log entries
  3. Adding a log entry → associates with current task and session
  4. Completing a task → logs completion and updates time reports

## Future Ideas

- GitHub integration
- Advanced charts and visualizations
- Team sharing
- Custom workflows and widget arrangements
- Pomodoro automation with breaks
- Mobile companion
- Full Obsidian integration

---

*This is our living roadmap - it will change as we learn.*
