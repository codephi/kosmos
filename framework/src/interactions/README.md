# Kosmos Framework - Módulo de Interações

## Visão Geral

O módulo de interações fornece uma biblioteca completa para manipulação de geometrias através de eventos do usuário no framework Kosmos, construído sobre o Bevy Engine.

## Funcionalidades

### 1. **Drag and Drop** 🖱️
Permite arrastar qualquer geometria marcada como `Draggable`.

**Como usar:**
- Clique e segure o botão esquerdo do mouse sobre uma geometria
- Mova o mouse para arrastar
- Solte o botão para finalizar o arraste

### 2. **Transform Mode** ✨
Criação interativa de geometrias com controle de tamanho e rotação baseado na posição do mouse.

**Controles:**
- `T` - Criar retângulo
- `G` - Criar círculo  
- `R` - Criar triângulo
- `H` - Criar hexágono
- `Y` - Criar estrela
- `Mouse` - Controla tamanho e rotação durante a criação
- `Click Esquerdo` - Confirma a criação
- `ESC` - Cancela a criação

### 3. **Selection** 🎯
Sistema de seleção visual com borda destacada.

**Como usar:**
- Clique em uma geometria para selecioná-la
- A geometria selecionada terá uma borda amarela
- Clique em outra geometria ou no espaço vazio para desselecionar

## Instalação

Adicione o plugin ao seu app Bevy:

```rust
use bevy::prelude::*;
use kosmos_framework::interactions::InteractionsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InteractionsPlugin)
        .add_systems(Startup, setup)
        .run();
}
```

## Componentes

### Componentes de Marcação

- `Draggable` - Marca uma entidade como arrastável
- `Selectable` - Marca uma entidade como selecionável
- `GeometryBounds { size: Vec2 }` - Define os limites para detecção de colisão

### Componentes de Estado

- `Dragging { offset: Vec2 }` - Estado ativo durante arraste
- `Selected` - Marca uma entidade como selecionada
- `Transforming` - Estado ativo durante criação de geometria
- `GeometryPreview` - Marca uma geometria em preview

## Eventos

O sistema emite eventos que podem ser capturados para lógica customizada:

```rust
fn handle_selection(
    mut events: EventReader<SelectEvent>
) {
    for event in events.read() {
        println!("Entidade selecionada: {:?}", event.entity);
    }
}
```

### Eventos Disponíveis

- `StartDragEvent { entity, start_position }`
- `StopDragEvent { entity, final_position }`
- `StartTransformEvent { geometry_data, start_position }`
- `ConfirmTransformEvent { entity }`
- `CancelTransformEvent`
- `SelectEvent { entity }`
- `DeselectEvent { entity }`

## Exemplo Completo

```rust
use bevy::prelude::*;
use kosmos_framework::{
    geometrics::Geometrics,
    interactions::prelude::*,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Câmera
    commands.spawn(Camera2d::default());
    
    // Criar uma geometria interativa
    let circle = Geometrics::circle(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::BLUE,
        50.0,
        Vec2::ZERO,
    );
    
    // Adicionar capacidades de interação
    commands.entity(circle).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(100.0) },
    ));
}
```

## Arquitetura

O módulo é estruturado de forma modular e extensível:

```
interactions/
├── mod.rs           # Plugin principal e configuração
├── components.rs    # Componentes reutilizáveis
├── events.rs        # Definição de eventos
├── input_utils.rs   # Utilitários de input e conversão
├── drag.rs          # Sistema de drag & drop
├── transform.rs     # Sistema de criação interativa
└── select.rs        # Sistema de seleção visual
```

### Extensibilidade

Cada funcionalidade é implementada como um plugin separado (`DragPlugin`, `TransformPlugin`, `SelectPlugin`), permitindo:

- Adicionar novos modos de interação sem modificar código existente
- Desabilitar funcionalidades específicas se necessário
- Substituir implementações por versões customizadas

## Modos de Interação

O sistema mantém um estado global `InteractionMode` que garante que apenas um modo esteja ativo por vez:

- `None` - Nenhuma interação ativa
- `Dragging` - Arrastando uma geometria
- `Transforming` - Criando uma nova geometria
- `Selecting` - Selecionando/geometria selecionada

## Performance

- Hit detection otimizado usando AABB (Axis-Aligned Bounding Box)
- Sistemas executam apenas quando necessário usando condições `run_if`
- Eventos permitem desacoplamento e processamento assíncrono

## Roadmap

Funcionalidades planejadas para versões futuras:

- [ ] Multi-seleção com box selection
- [ ] Rotação e escala de objetos selecionados
- [ ] Snap to grid
- [ ] Undo/Redo
- [ ] Copy/Paste de geometrias
- [ ] Grupos e hierarquia de objetos
- [ ] Constraints e alinhamento automático

## Licença

Este módulo faz parte do Kosmos Framework e segue a mesma licença do projeto principal.