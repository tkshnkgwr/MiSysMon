# System Diagram

```mermaid
graph TD
    subgraph "OS (Windows/Linux)"
        SYS[System Resources]
        WIN[Windows OS Named Mutex]
    end

    subgraph "Mini System Monitor (Rust/egui)"
        APP[App Loop]
        SI[sysinfo 0.38]
        CH[chrono]
        UI[egui Renderer]
        CL[common_lib]
        
        APP -->|1s Interval| SI
        APP -->|Time Fetch| CH
        SI -->|Fetch| SYS
        SI -->|Data| APP
        CH -->|Time String| APP
        APP -->|State| UI
        UI -->|Draw| SCR[Screen]
        APP -->|Single Instance Check| CL
    end

    subgraph "User Interaction"
        DRAG[Drag Grip] -->|Move Window| APP
        CLOSE[Close Button] -->|Terminate| APP
    end

    CL -->|Create/Acquire Mutex| WIN
```
