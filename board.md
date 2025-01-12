``` mermaid
classDiagram

direction TB

class Application {
    +run(terminal: Terminal, uiRenderer: UIRenderer): Result
}

class ComponentManager {
    +addComponent(component: UIComponent)
    +removeComponent(index: int)
    +render(frame: Frame)
    +handleEvent(event: Event)
    -components: List<UIComponent>
}

class ScreenManager {
    +transitionTo(screen: Screen)
    +render(terminal: Terminal, uiRenderer: UIRenderer): Result
    +handleEvent(event: Event, uiRenderer: UIRenderer): Screen
    -current: Screen
    -state: State
}

class Screen {
    +setupUI(): UIRenderer
    +render(terminal: Terminal, uiRenderer: UIRenderer): Result
    +handleEvent(event: Event, state: State, uiRenderer: UIRenderer): Screen
}

class UIRenderer {
    +addComponent(component: UIComponent)
    +removeComponent(index: int)
    +render(frame: Frame)
    +handleEvent(event: Event)
    -components: List<UIComponent>
}

class UIComponent {
    <<interface>>
    +render(frame: Frame)
    +handleEvent(event: Event): Result
}

class TextBlock {
    +new(text: String, area: Rect, active: bool)
    +setActive(active: bool)
    +isActive(): bool
    +render(frame: Frame)
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

Application --> ScreenManager
ScreenManager --> Screen
Screen --> UIRenderer
Screen --> State
UIRenderer --> UIComponent
UIComponent <|-- TextBlock
State <|-- InMemoryState
```
