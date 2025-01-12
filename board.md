# Application Structure

This document contains the class diagram that represents the application's structure.

```mermaid
---
title: Application Structure
---
classDiagram
direction TB

%% Main application entry point
class Application {
    <<entry-point>>
    +run(interface: UI, renderUI: UIRenderer): Result
}

%% UI renderer and its responsibilities
class UIRenderer {
    <<service>>
    +addComponent(component: UIComponent)
    +removeComponent(index: int)
    +render(context: RenderingContext)
    +handleEvent(event: Event)
    -components: List<UIComponent>
}

%% Manages screens and state transitions
class ScreenManager {
    <<controller>>
    +transitionTo(screen: Screen)
    +render(interface: UI, renderUI: UIRenderer): Result
    +handleEvent(event: Event, renderUI: UIRenderer): Screen
    -current: Screen
    -state: State
}

%% Represents a screen in the application
class Screen {
    <<abstract>>
    +setupUI(): UIRenderer
    +render(interface: UI, renderUI: UIRenderer): Result
    +handleEvent(event: Event, state: State, renderUI: UIRenderer): Screen
}

%% Base UI component interface
class UIComponent {
    <<interface>>
    +render(context: RenderingContext)
    +handleEvent(event: Event): Result
}

%% A specific UI component for displaying text
class TextBlock {
    <<component>>
    +new(text: String, area: Rect, active: bool)
    +setActive(active: bool)
    +isActive(): bool
    +render(context: RenderingContext)
    +handleEvent(event: Event): Result
    -text: String
    -area: Rect
    -active: bool
}

%% State management interface
class State {
    <<interface>>
    +set(key: String, value: String)
    +get(key: String): String
    +clear()
    +setActive(active: bool)
    +isActive(): bool
}

%% Implementation of state using in-memory storage
class InMemoryState {
    <<data-store>>
    +set(key: String, value: String)
    +get(key: String): String
    +clear()
    +setActive(active: bool)
    +isActive(): bool
    -state: HashMap<String, String>
    -active: bool
}

%% Rendering context description
class RenderingContext {
    <<description>>
    The context used to draw or display UI elements and manage their layout.
}

%% Relationships
Application --> ScreenManager : "Manages screens"
ScreenManager --> Screen : "Transitions to"
Screen --> UIRenderer : "Uses for rendering"
Screen --> State : "Depends on"
UIRenderer --> UIComponent : "Contains"
UIComponent <|-- TextBlock : "Specialized"
State <|-- InMemoryState : "Implementation of"
UIComponent --> RenderingContext : "Uses"
