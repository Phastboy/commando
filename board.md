``` mermaid
classDiagram
direction TB

class Application {
    +run(interface: UI, renderUI: UIRenderer): Result
}

class UIRenderer {
    +addComponent(component: UIComponent)
    +removeComponent(index: int)
    +render(context: RenderingContext)
    +handleEvent(event: Event)
    -components: List<UIComponent>
}

class ScreenManager {
    +transitionTo(screen: Screen)
    +render(interface: UI, renderUI: UIRenderer): Result
    +handleEvent(event: Event, renderUI: UIRenderer): Screen
    -current: Screen
    -state: State
}

class Screen {
    +setupUI(): UIRenderer
    +render(interface: UI, renderUI: UIRenderer): Result
    +handleEvent(event: Event, state: State, renderUI: UIRenderer): Screen
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
Screen --> UIRenderer
Screen --> State
UIRenderer --> UIComponent
UIComponent <|-- TextBlock
State <|-- InMemoryState
UIComponent --> RenderingContext
```
