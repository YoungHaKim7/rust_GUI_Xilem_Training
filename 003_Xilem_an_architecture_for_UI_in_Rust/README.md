# Xilem: an architecture for UI in Rust

- https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html



<table>
    <tr>
    <td colspan="2" align="center">Xilem과 다른 GUI의 차이점
</td>
    </tr>
<tr align="center">
  <td>Xilem</td>
  <td>$${\color{green} \verb|**|The Elm<br>Architecture}$$</td>
</tr>
<tr align="center">
  <td>Synchronized trees</td>
  <td>$${\color{green} \verb|**|does not require shared mutable state}$$</td>
</tr>
<tr align="center">
  <td>Xilem</td>
  <td>$${\color{green} \verb|**|iced, relm, and Vizia_일부 사용}$$</td>
</tr>
<tr align="center">
  <td>The Xilem architecture는 트리를 생성하고 동기화를 유지하는 것을 기반으로 함. 이러한 방식으로 이전 블로그 게시물인 <a href="https://raphlinus.github.io/ui/druid/2019/11/22/reactive-ui.html">'반응형 UI의 통일된 이론을 향해'Towards principled reactive UI</a> 에서 설명한 아디어를 개선한 것</td>
  <td>$${\color{green} \verb|**|일반적으로 잘 작동하시만, 명시적인 메시지 유형을 생성하고 이를 디스패치해야 할 필요성은 모호하며, Elm 아키텍처는 구성 요소와 다른 아키텍처를 깔끔하게 인수분해하지 않음. 복잡할수록 ㅠㅠ 망함.}$$</td>
</tr>
</table>


<hr />

# The Elm Architecture이게 젤 중요한 개념
- A particularly common architecture for UI in Rust is The Elm Architecture, which also does not require shared mutable state.
  - Rust에서 UI를 위한 특히 일반적인 아키텍처는 Elm Architecture로, 공유 가능한 상태가 필요하지 않습니다.

|||
|-|-|
|relatively pure form and<br /> in a modified form. |immediate mode|
|-|egui|


|The Elm architecture<br>영향을 받은거|
|-|
|[Iced](https://github.com/iced-rs/iced), [Relm](https://github.com/antoyo/relm), [Vizia](https://github.com/vizia/vizia)|

- Rather, to support interactions from the UI, gestures and other related UI actions creates messages which are then sent to an update method which takes central app state. Iced, relm, and Vizia all use some form of this architecture. Generally it works well, but the need to create an explicit message type and dispatch on it is verbose, and the Elm architecture does not support cleanly factored components as well as some other architectures. The Elm documentation specifically warns against components, saying, “actively trying to make components is a recipe for disaster in Elm.”
  - 히려 UI의 상호작용을 지원하기 위해 제스처 및 기타 관련 UI 작업은 메시지를 생성하고, 이 메시지는 중앙 앱 상태를 취하는 업데이트 메서드로 전송됩니다. Ice, Relm, Vizia는 모두 이 아키텍처의 일부 형태를 사용합니다. 일반적으로 잘 작동하지만 명시적인 메시지 유형을 생성하고 이를 디스패치해야 할 필요성은 모호하며, Elm 아키텍처는 깨끗한 요소를 포함한 일부 아키텍처를 지원하지 않습니다. Elm 문서는 특히 구성 요소에 대해 경고하며, "구성 요소를 적극적으로 만들려고 하는 것은 Elm의 재앙을 초래하는 레시피"라고 말합니다

- A particularly common architecture for UI in Rust is The Elm Architecture, which also does not require shared mutable state. Rather, to support interactions from the UI, gestures and other related UI actions creates messages which are then sent to an update method which takes central app state. Iced, relm, and Vizia all use some form of this architectur 
  - Rust에서 UI를 위한 특히 일반적인 아키텍처는 Elm Architecture로, 공유 가능한 상태를 요구하지 않습니다. 오히려 UI로부터의 상호작용을 지원하기 위해 제스처 및 기타 관련 UI 동작이 메시지를 생성하고, 이 메시지는 중앙 앱 상태를 취하는 업데이트 방법으로 전송됩니다. Iceed, Relm, Vizia 모두 이 아키텍처의 일부 형태를 사용합니다

<hr />

|Immediate mode|
|-|
|[egui](https://github.com/emilk/egui), [makepad](https://github.com/makepad/makepad)|


- Another common architecture is immediate mode GUI, both in a relatively pure form and in a modified form. It is popular in Rust because it doesn’t require shared mutable state. It also benefits from overall system simplicity. However, the model is oversimplified in a number of ways, and it is difficult to do sophisticated layout and other patterns that are easier in retained widget systems. There are also numerous papercuts related to sometimes rendering stale state. (I experimented with a retained widget backend emulating an immediate mode API in the “crochet” architecture experiment and concluded that the result was not compelling). The popular egui crate is solidly an implementation of immediate mode, and makepad is also based on it, though it differs in some important ways.
  - 또 다른 일반적인 아키텍처는 비교적 순수한 형태와 수정된 형태의 즉시 모드 GUI입니다. 이 아키텍처는 공유된 돌연변이 상태가 필요하지 않기 때문에 Rust에서 인기가 많습니다. 또한 전체 시스템 단순성의 이점도 있습니다. 그러나 이 모델은 여러 가지 면에서 지나치게 단순화되어 있으며, 유지 위젯 시스템에서 더 쉬운 정교한 레이아웃 및 기타 패턴을 수행하기가 어렵습니다. 때때로 오래된 상태를 렌더링하는 것과 관련된 수많은 페이퍼컷도 있습니다. (저는 "크로셰" 아키텍처 실험에서 즉시 모드 API를 모방한 유지 위젯 백엔드를 실험해 본 결과, 그 결과가 설득력이 없다는 결론을 내렸습니다. 인기 있는 egui 상자는 즉시 모드를 구현한 것으로, makepad도 이를 기반으로 하지만 몇 가지 중요한 점에서 다릅니다.
