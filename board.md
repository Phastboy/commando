``` mermaid
classDiagram
direction TB

class Application {
    +run(interface: UI, componentManager: ComponentManager): Result
}

class ComponentManager {
    +addComponent(component: UIComponent)
    +removeComponent(index: int)
    +render(context: RenderingContext)
    +handleEvent(event: Event)
    -components: List<UIComponent>
}

class ScreenManager {
    +transitionTo(screen: Screen)
    +render(interface: UI, componentManager: ComponentManager): Result
    +handleEvent(event: Event, componentManager: ComponentManager): Screen
    -current: Screen
    -state: State
}

class Screen {
    +setupUI(): ComponentManager
    +render(interface: UI, componentManager: ComponentManager): Result
    +handleEvent(event: Event, state: State, componentManager: ComponentManager): Screen
}

class UIComponent {
    <<interface>>
    +render(context: RenderingContext)
    +handleEvent(event: Event): Result
}

class TextBlock {
    +new(text: String, area: Rect, active: bool)
    +setActive(active: bool)
    +isActive(): bool
    +render(context: RenderingContext)
    +handleEvent(event: Event): Result
    -text: String
    -area: Rect
    -active: bool
}

class State {
    <<interface>>
    +set(key: String, value: String)
    +get(key: String): String
    +clear()
    +setActive(active: bool)
    +isActive(): bool
}

class InMemoryState {
    +set(key: String, value: String)
    +get(key: String): String
    +clear()
    +setActive(active: bool)
    +isActive(): bool
    -state: HashMap<String, String>
    -active: bool
}

class RenderingContext {
    <<description>>
    The context used to draw or display UI elements and manage their layout.
}

Application --> ScreenManager
ScreenManager --> Screen
Screen --> ComponentManager
Screen --> State
ComponentManager --> UIComponent
UIComponent <|-- TextBlock
State <|-- InMemoryState
UIComponent --> RenderingContext
```
