use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ANIMATIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "back-in-down".to_string(),
        r#"@keyframes back-in-down {
  0% {
    transform: translateY(-1200px) scale(0.7);
    opacity: 0.7;
  }

  80% {
    transform: translateY(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-in-down;
}
"#
        .to_string(),
    );
    map.insert(
        "back-in-left".to_string(),
        r#"@keyframes back-in-left {
  0% {
    transform: translateX(-2000px) scale(0.7);
    opacity: 0.7;
  }

  80% {
    transform: translateX(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "back-in-right".to_string(),
        r#"@keyframes back-in-right {
  0% {
    transform: translateX(2000px) scale(0.7);
    opacity: 0.7;
  }

  80% {
    transform: translateX(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "back-in-up".to_string(),
        r#"@keyframes back-in-up {
  0% {
    transform: translateY(1200px) scale(0.7);
    opacity: 0.7;
  }

  80% {
    transform: translateY(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-in-up;
}
"#
        .to_string(),
    );
    map.insert(
        "back-out-down".to_string(),
        r#"@keyframes back-out-down {
  0% {
    transform: scale(1);
    opacity: 1;
  }

  20% {
    transform: translateY(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: translateY(700px) scale(0.7);
    opacity: 0.7;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-out-down;
}
"#
        .to_string(),
    );
    map.insert(
        "back-out-left".to_string(),
        r#"@keyframes back-out-left {
  0% {
    transform: scale(1);
    opacity: 1;
  }

  20% {
    transform: translateX(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: translateX(-2000px) scale(0.7);
    opacity: 0.7;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "back-out-right".to_string(),
        r#"@keyframes back-out-right {
  0% {
    transform: scale(1);
    opacity: 1;
  }

  20% {
    transform: translateX(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: translateX(2000px) scale(0.7);
    opacity: 0.7;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "back-out-up".to_string(),
        r#"@keyframes back-out-up {
  0% {
    transform: scale(1);
    opacity: 1;
  }

  20% {
    transform: translateY(0px) scale(0.7);
    opacity: 0.7;
  }

  100% {
    transform: translateY(-700px) scale(0.7);
    opacity: 0.7;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: back-out-up;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-bl".to_string(),
        r#"@keyframes bg-pan-bl {
    0% {
        background-position: 100% 0%
    }
    to {
        background-position: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-bottom".to_string(),
        r#"@keyframes bg-pan-bottom {
    0% {
        background-position: 50% 0%
    }
    to {
        background-position: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-br".to_string(),
        r#"@keyframes bg-pan-br {
    0% {
        background-position: 0% 0%
    }
    to {
        background-position: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-br;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-left".to_string(),
        r#"@keyframes bg-pan-left {
    0% {
        background-position: 100% 50%
    }
    to {
        background-position: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-left;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-right".to_string(),
        r#"@keyframes bg-pan-right {
    0% {
        background-position: 0% 50%
    }
    to {
        background-position: 100% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-right;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-tl".to_string(),
        r#"@keyframes bg-pan-tl {
    0% {
        background-position: 100% 100%
    }
    to {
        background-position: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-top".to_string(),
        r#"@keyframes bg-pan-top {
    0% {
        background-position: 50% 100%
    }
    to {
        background-position: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-top;
}
"#
        .to_string(),
    );
    map.insert(
        "bg-pan-tr".to_string(),
        r#"@keyframes bg-pan-tr {
    0% {
        background-position: 0% 100%
    }
    to {
        background-position: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bg-pan-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "blink-1".to_string(),
        r#"@keyframes blink-1 {
    0%,
    50%,
    to {
        opacity: 1
    }
    25%,
    75% {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blink-1;
}
"#
        .to_string(),
    );
    map.insert(
        "blink-2".to_string(),
        r#"@keyframes blink-2 {
    0%,
    to {
        opacity: 1
    }
    50% {
        opacity: .2
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blink-2;
}
"#
        .to_string(),
    );
    map.insert(
        "blur-out-contract-bck".to_string(),
        r#"@keyframes blur-out-contract-bck {
    0% {
        transform: translateZ(0);
        filter: blur(.01)
    }
    to {
        letter-spacing: -.5em;
        transform: translateZ(-500px);
        filter: blur(12px) opacity(0%)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blur-out-contract-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "blur-out-contract".to_string(),
        r#"@keyframes blur-out-contract {
    0% {
        filter: blur(.01)
    }
    to {
        letter-spacing: -.5em;
        filter: blur(12px) opacity(0%)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blur-out-contract;
}
"#
        .to_string(),
    );
    map.insert(
        "blur-out-expand-fwd".to_string(),
        r#"@keyframes blur-out-expand-fwd {
    0% {
        transform: translateZ(0);
        filter: blur(.01)
    }
    to {
        letter-spacing: 1em;
        transform: translateZ(300px);
        filter: blur(12px) opacity(0%)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blur-out-expand-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "blur-out-expand".to_string(),
        r#"@keyframes blur-out-expand {
    0% {
        filter: blur(.01)
    }
    to {
        letter-spacing: 1em;
        filter: blur(12px) opacity(0%)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: blur-out-expand;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-bottom".to_string(),
        r#"@keyframes bounce-bottom {
    0% {
        transform: translateY(45px);
        animation-timing-function: ease-in;
        opacity: 1
    }
    24% {
        opacity: 1
    }
    40% {
        transform: translateY(24px);
        animation-timing-function: ease-in
    }
    65% {
        transform: translateY(12px);
        animation-timing-function: ease-in
    }
    82% {
        transform: translateY(6px);
        animation-timing-function: ease-in
    }
    93% {
        transform: translateY(4px);
        animation-timing-function: ease-in
    }
    25%,
    55%,
    75%,
    87% {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    to {
        transform: translateY(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-bck".to_string(),
        r#"@keyframes bounce-in-bck {
    0% {
        transform: scale(7);
        animation-timing-function: ease-in;
        opacity: 0
    }
    38% {
        transform: scale(1);
        animation-timing-function: ease-out;
        opacity: 1
    }
    55% {
        transform: scale(1.5);
        animation-timing-function: ease-in
    }
    72%,
    89%,
    to {
        transform: scale(1);
        animation-timing-function: ease-out
    }
    81% {
        transform: scale(1.24);
        animation-timing-function: ease-in
    }
    95% {
        transform: scale(1.04);
        animation-timing-function: ease-in
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-bottom".to_string(),
        r#"@keyframes bounce-in-bottom {
    0% {
        transform: translateY(500px);
        animation-timing-function: ease-in;
        opacity: 0
    }
    38% {
        transform: translateY(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
    55% {
        transform: translateY(65px);
        animation-timing-function: ease-in
    }
    72%,
    90%,
    to {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    81% {
        transform: translateY(28px);
        animation-timing-function: ease-in
    }
    95% {
        transform: translateY(8px);
        animation-timing-function: ease-in
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-down".to_string(),
        r#"@keyframes bounce-in-down {
  from,
  60%,
  75%,
  90%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
  }

  0% {
    opacity: 0;
    transform: translate3d(0, -3000px, 0) scaleY(3);
  }

  60% {
    opacity: 1;
    transform: translate3d(0, 25px, 0) scaleY(0.9);
  }

  75% {
    transform: translate3d(0, -10px, 0) scaleY(0.95);
  }

  90% {
    transform: translate3d(0, 5px, 0) scaleY(0.985);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-down;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-fwd".to_string(),
        r#"@keyframes bounce-in-fwd {
    0% {
        transform: scale(0);
        animation-timing-function: ease-in;
        opacity: 0
    }
    38% {
        transform: scale(1);
        animation-timing-function: ease-out;
        opacity: 1
    }
    55% {
        transform: scale(.7);
        animation-timing-function: ease-in
    }
    72%,
    89%,
    to {
        transform: scale(1);
        animation-timing-function: ease-out
    }
    81% {
        transform: scale(.84);
        animation-timing-function: ease-in
    }
    95% {
        transform: scale(.95);
        animation-timing-function: ease-in
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-left".to_string(),
        r#"@keyframes bounce-in-left {
  from,
  60%,
  75%,
  90%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
  }

  0% {
    opacity: 0;
    transform: translate3d(-3000px, 0, 0) scaleX(3);
  }

  60% {
    opacity: 1;
    transform: translate3d(25px, 0, 0) scaleX(1);
  }

  75% {
    transform: translate3d(-10px, 0, 0) scaleX(0.98);
  }

  90% {
    transform: translate3d(5px, 0, 0) scaleX(0.995);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-right".to_string(),
        r#"@keyframes bounce-in-right {
  from,
  60%,
  75%,
  90%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
  }

  from {
    opacity: 0;
    transform: translate3d(3000px, 0, 0) scaleX(3);
  }

  60% {
    opacity: 1;
    transform: translate3d(-25px, 0, 0) scaleX(1);
  }

  75% {
    transform: translate3d(10px, 0, 0) scaleX(0.98);
  }

  90% {
    transform: translate3d(-5px, 0, 0) scaleX(0.995);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-top".to_string(),
        r#"@keyframes bounce-in-top {
    0% {
        transform: translateY(-500px);
        animation-timing-function: ease-in;
        opacity: 0
    }
    38% {
        transform: translateY(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
    55% {
        transform: translateY(-65px);
        animation-timing-function: ease-in
    }
    72%,
    90%,
    to {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    81% {
        transform: translateY(-28px);
        transform: translateY(-8px);
        animation-timing-function: ease-in
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in-up".to_string(),
        r#"@keyframes bounce-in-up {
  from,
  60%,
  75%,
  90%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
  }

  from {
    opacity: 0;
    transform: translate3d(0, 3000px, 0) scaleY(5);
  }

  60% {
    opacity: 1;
    transform: translate3d(0, -20px, 0) scaleY(0.9);
  }

  75% {
    transform: translate3d(0, 10px, 0) scaleY(0.95);
  }

  90% {
    transform: translate3d(0, -5px, 0) scaleY(0.985);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-in-up;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-in".to_string(),
        r#"@keyframes bounce-in {
  from,
  20%,
  40%,
  60%,
  80%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
  }

  0% {
    opacity: 0;
    transform: scale3d(0.3, 0.3, 0.3);
  }

  20% {
    transform: scale3d(1.1, 1.1, 1.1);
  }

  40% {
    transform: scale3d(0.9, 0.9, 0.9);
  }

  60% {
    opacity: 1;
    transform: scale3d(1.03, 1.03, 1.03);
  }

  80% {
    transform: scale3d(0.97, 0.97, 0.97);
  }

  to {
    opacity: 1;
    transform: scale3d(1, 1, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-duration: 0.75s;
  animation-name: bounce-in;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-left".to_string(),
        r#"@keyframes bounce-left {
    0% {
        transform: translateX(-48px);
        animation-timing-function: ease-in;
        opacity: 1
    }
    24% {
        opacity: 1
    }
    40% {
        transform: translateX(-26px);
        animation-timing-function: ease-in
    }
    65% {
        transform: translateX(-13px);
        animation-timing-function: ease-in
    }
    82% {
        transform: translateX(-6.5px);
        animation-timing-function: ease-in
    }
    93% {
        transform: translateX(-4px);
        animation-timing-function: ease-in
    }
    25%,
    55%,
    75%,
    87%,
    98% {
        transform: translateX(0);
        animation-timing-function: ease-out
    }
    to {
        transform: translateX(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-left;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-bck".to_string(),
        r#"@keyframes bounce-out-bck {
    0%,
    15%,
    38% {
        transform: translateZ(0);
        animation-timing-function: ease-out
    }
    5% {
        transform: translateZ(-100px);
        animation-timing-function: ease-in
    }
    25% {
        transform: translateZ(-110px);
        animation-timing-function: ease-in
    }
    52% {
        transform: translateZ(-200px);
        animation-timing-function: ease-in
    }
    70% {
        transform: translateZ(0) scale(1);
        animation-timing-function: ease-out
    }
    85% {
        opacity: 1
    }
    to {
        transform: translateZ(-900px) scale(0);
        animation-timing-function: ease-in;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-bottom".to_string(),
        r#"@keyframes bounce-out-bottom {
    0%,
    15%,
    38%,
    70% {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    5% {
        transform: translateY(30px);
        animation-timing-function: ease-in
    }
    25% {
        transform: translateY(38px);
        animation-timing-function: ease-in
    }
    52% {
        transform: translateY(75px);
        animation-timing-function: ease-in
    }
    85% {
        opacity: 1
    }
    to {
        transform: translateY(800px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-down".to_string(),
        r#"@keyframes bounce-out-down {
  20% {
    transform: translate3d(0, 10px, 0) scaleY(0.985);
  }

  40%,
  45% {
    opacity: 1;
    transform: translate3d(0, -20px, 0) scaleY(0.9);
  }

  to {
    opacity: 0;
    transform: translate3d(0, 2000px, 0) scaleY(3);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-down;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-fwd".to_string(),
        r#"@keyframes bounce-out-fwd {
    0%,
    15%,
    38%,
    70% {
        transform: translateZ(0);
        animation-timing-function: ease-out
    }
    5% {
        transform: translateZ(90px);
        animation-timing-function: ease-in
    }
    25% {
        transform: translateZ(95px);
        animation-timing-function: ease-in
    }
    52% {
        transform: translateZ(150px);
        animation-timing-function: ease-in
    }
    85% {
        opacity: 1
    }
    to {
        transform: translateZ(500px);
        animation-timing-function: ease-in;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-left".to_string(),
        r#"@keyframes bounce-out-left {
  20% {
    opacity: 1;
    transform: translate3d(20px, 0, 0) scaleX(0.9);
  }

  to {
    opacity: 0;
    transform: translate3d(-2000px, 0, 0) scaleX(2);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-right".to_string(),
        r#"@keyframes bounce-out-right {
  20% {
    opacity: 1;
    transform: translate3d(-20px, 0, 0) scaleX(0.9);
  }

  to {
    opacity: 0;
    transform: translate3d(2000px, 0, 0) scaleX(2);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-top".to_string(),
        r#"@keyframes bounce-out-top {
    0%,
    15%,
    38%,
    70% {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    5% {
        transform: translateY(-30px);
        animation-timing-function: ease-in
    }
    25% {
        transform: translateY(-38px);
        animation-timing-function: ease-in
    }
    52% {
        transform: translateY(-75px);
        animation-timing-function: ease-in
    }
    85% {
        opacity: 1
    }
    to {
        transform: translateY(-800px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out-up".to_string(),
        r#"@keyframes bounce-out-up {
  20% {
    transform: translate3d(0, -10px, 0) scaleY(0.985);
  }

  40%,
  45% {
    opacity: 1;
    transform: translate3d(0, 20px, 0) scaleY(0.9);
  }

  to {
    opacity: 0;
    transform: translate3d(0, -2000px, 0) scaleY(3);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-out-up;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-out".to_string(),
        r#"@keyframes bounce-out {
  20% {
    transform: scale3d(0.9, 0.9, 0.9);
  }

  50%,
  55% {
    opacity: 1;
    transform: scale3d(1.1, 1.1, 1.1);
  }

  to {
    opacity: 0;
    transform: scale3d(0.3, 0.3, 0.3);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-duration: 0.75s;
  animation-name: bounce-out;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-right".to_string(),
        r#"@keyframes bounce-right {
    0% {
        transform: translateX(48px);
        animation-timing-function: ease-in;
        opacity: 1
    }
    24% {
        opacity: 1
    }
    40% {
        transform: translateX(26px);
        animation-timing-function: ease-in
    }
    65% {
        transform: translateX(13px);
        animation-timing-function: ease-in
    }
    82% {
        transform: translateX(6.5px);
        animation-timing-function: ease-in
    }
    93% {
        transform: translateX(4px);
        animation-timing-function: ease-in
    }
    25%,
    55%,
    75%,
    87%,
    98% {
        transform: translateX(0);
        animation-timing-function: ease-out
    }
    to {
        transform: translateX(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-right;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce-top".to_string(),
        r#"@keyframes bounce-top {
    0% {
        transform: translateY(-45px);
        animation-timing-function: ease-in;
        opacity: 1
    }
    24% {
        opacity: 1
    }
    40% {
        transform: translateY(-24px);
        animation-timing-function: ease-in
    }
    65% {
        transform: translateY(-12px);
        animation-timing-function: ease-in
    }
    82% {
        transform: translateY(-6px);
        animation-timing-function: ease-in
    }
    93% {
        transform: translateY(-4px);
        animation-timing-function: ease-in
    }
    25%,
    55%,
    75%,
    87% {
        transform: translateY(0);
        animation-timing-function: ease-out
    }
    to {
        transform: translateY(0);
        animation-timing-function: ease-out;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce-top;
}
"#
        .to_string(),
    );
    map.insert(
        "bounce".to_string(),
        r#"@keyframes bounce {
  from,
  20%,
  53%,
  to {
    animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
    transform: translate3d(0, 0, 0);
  }

  40%,
  43% {
    animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
    transform: translate3d(0, -30px, 0) scaleY(1.1);
  }

  70% {
    animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
    transform: translate3d(0, -15px, 0) scaleY(1.05);
  }

  80% {
    transition-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
    transform: translate3d(0, 0, 0) scaleY(0.95);
  }

  90% {
    transform: translate3d(0, -4px, 0) scaleY(1.02);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: bounce;
  transform-origin: center bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "color-change-2x".to_string(),
        r#"@keyframes color-change-2x {
    0% {
        background: #19dcea
    }
    to {
        background: #b22cff
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: color-change-2x;
}
"#
        .to_string(),
    );
    map.insert(
        "color-change-3x".to_string(),
        r#"@keyframes color-change-3x {
    0% {
        background: #19dcea
    }
    50% {
        background: #b22cff
    }
    to {
        background: #ea2222
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: color-change-3x;
}
"#
        .to_string(),
    );
    map.insert(
        "color-change-4x".to_string(),
        r#"@keyframes color-change-4x {
    0% {
        background: #19dcea
    }
    33.3333% {
        background: #b22cff
    }
    66.666% {
        background: #ea2222
    }
    to {
        background: #f5be10
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: color-change-4x;
}
"#
        .to_string(),
    );
    map.insert(
        "color-change-5x".to_string(),
        r#"@keyframes color-change-5x {
    0% {
        background: #19dcea
    }
    25% {
        background: #b22cff
    }
    50% {
        background: #ea2222
    }
    75% {
        background: #f5be10
    }
    to {
        background: #3bd80d
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: color-change-5x;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-bck".to_string(),
        r#"@keyframes fade-in-bck {
    0% {
        transform: translateZ(80px);
        opacity: 0
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-bl".to_string(),
        r#"@keyframes fade-in-bl {
    0% {
        transform: translateX(-50px) translateY(50px);
        opacity: 0
    }
    to {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-bottom-left".to_string(),
        r#"@keyframes fade-in-bottom-left {
  from {
    opacity: 0;
    transform: translate3d(-100%, 100%, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-bottom-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-bottom-right".to_string(),
        r#"@keyframes fade-in-bottom-right {
  from {
    opacity: 0;
    transform: translate3d(100%, 100%, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-bottom-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-bottom".to_string(),
        r#"@keyframes fade-in-bottom {
    0% {
        transform: translateY(50px);
        opacity: 0
    }
    to {
        transform: translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-br".to_string(),
        r#"@keyframes fade-in-br {
    0% {
        transform: translateX(50px) translateY(50px);
        opacity: 0
    }
    to {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-br;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-down-big".to_string(),
        r#"@keyframes fade-in-down-big {
  from {
    opacity: 0;
    transform: translate3d(0, -2000px, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-down-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-down".to_string(),
        r#"@keyframes fade-in-down {
  from {
    opacity: 0;
    transform: translate3d(0, -100%, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-down;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-fwd".to_string(),
        r#"@keyframes fade-in-fwd {
    0% {
        transform: translateZ(-80px);
        opacity: 0
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-left-big".to_string(),
        r#"@keyframes fade-in-left-big {
  from {
    opacity: 0;
    transform: translate3d(-2000px, 0, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-left-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-left".to_string(),
        r#"@keyframes fade-in-left {
  from {
    opacity: 0;
    transform: translate3d(-100%, 0, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-right-big".to_string(),
        r#"@keyframes fade-in-right-big {
  from {
    opacity: 0;
    transform: translate3d(2000px, 0, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-right-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-right".to_string(),
        r#"@keyframes fade-in-right {
  from {
    opacity: 0;
    transform: translate3d(100%, 0, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-tl".to_string(),
        r#"@keyframes fade-in-tl {
    0% {
        transform: translateX(-50px) translateY(-50px);
        opacity: 0
    }
    to {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-top-left".to_string(),
        r#"@keyframes fade-in-top-left {
  from {
    opacity: 0;
    transform: translate3d(-100%, -100%, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-top-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-top-right".to_string(),
        r#"@keyframes fade-in-top-right {
  from {
    opacity: 0;
    transform: translate3d(100%, -100%, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-top-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-top".to_string(),
        r#"@keyframes fade-in-top {
    0% {
        transform: translateY(-50px);
        opacity: 0
    }
    to {
        transform: translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-tr".to_string(),
        r#"@keyframes fade-in-tr {
    0% {
        transform: translateX(50px) translateY(-50px);
        opacity: 0
    }
    to {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-up-big".to_string(),
        r#"@keyframes fade-in-up-big {
  from {
    opacity: 0;
    transform: translate3d(0, 2000px, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-up-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in-up".to_string(),
        r#"@keyframes fade-in-up {
  from {
    opacity: 0;
    transform: translate3d(0, 100%, 0);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in-up;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-in".to_string(),
        r#"@keyframes fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-in;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-bck".to_string(),
        r#"@keyframes fade-out-bck {
    0% {
        transform: translateZ(0);
        opacity: 1
    }
    to {
        transform: translateZ(-80px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-bl".to_string(),
        r#"@keyframes fade-out-bl {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateX(-50px) translateY(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-bottom-left".to_string(),
        r#"@keyframes fade-out-bottom-left {
  from {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
  to {
    opacity: 0;
    transform: translate3d(-100%, 100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-bottom-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-bottom-right".to_string(),
        r#"@keyframes fade-out-bottom-right {
  from {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
  to {
    opacity: 0;
    transform: translate3d(100%, 100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-bottom-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-bottom".to_string(),
        r#"@keyframes fade-out-bottom {
    0% {
        transform: translateY(0);
        opacity: 1
    }
    to {
        transform: translateY(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-br".to_string(),
        r#"@keyframes fade-out-br {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateX(50px) translateY(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-br;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-down-big".to_string(),
        r#"@keyframes fade-out-down-big {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(0, 2000px, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-down-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-down".to_string(),
        r#"@keyframes fade-out-down {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(0, 100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-down;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-fwd".to_string(),
        r#"@keyframes fade-out-fwd {
    0% {
        transform: translateZ(0);
        opacity: 1
    }
    to {
        transform: translateZ(80px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-left-big".to_string(),
        r#"@keyframes fade-out-left-big {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(-2000px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-left-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-left".to_string(),
        r#"@keyframes fade-out-left {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(-100%, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-right-big".to_string(),
        r#"@keyframes fade-out-right-big {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(2000px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-right-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-right".to_string(),
        r#"@keyframes fade-out-right {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(100%, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-tl".to_string(),
        r#"@keyframes fade-out-tl {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateX(-50px) translateY(-50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-top-left".to_string(),
        r#"@keyframes fade-out-top-left {
  from {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
  to {
    opacity: 0;
    transform: translate3d(-100%, -100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-top-left;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-top-right".to_string(),
        r#"@keyframes fade-out-top-right {
  from {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
  to {
    opacity: 0;
    transform: translate3d(100%, -100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-top-right;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-top".to_string(),
        r#"@keyframes fade-out-top {
    0% {
        transform: translateY(0);
        opacity: 1
    }
    to {
        transform: translateY(-50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-tr".to_string(),
        r#"@keyframes fade-out-tr {
    0% {
        transform: translateX(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateX(50px) translateY(-50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-up-big".to_string(),
        r#"@keyframes fade-out-up-big {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(0, -2000px, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-up-big;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out-up".to_string(),
        r#"@keyframes fade-out-up {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(0, -100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out-up;
}
"#
        .to_string(),
    );
    map.insert(
        "fade-out".to_string(),
        r#"@keyframes fade-out {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: fade-out;
}
"#
        .to_string(),
    );
    map.insert(
        "flash".to_string(),
        r#"@keyframes flash {
  from,
  50%,
  to {
    opacity: 1;
  }

  25%,
  75% {
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flash;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-1".to_string(),
        r#"@keyframes flicker-1 {
    0%,
    41.99%,
    43.01%,
    47.99%,
    49.01%,
    to {
        opacity: 1
    }
    42%,
    43%,
    48%,
    49% {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-2".to_string(),
        r#"@keyframes flicker-2 {
    0%,
    41.99%,
    43.01%,
    45.99%,
    46.91%,
    51.99%,
    52.81%,
    to {
        opacity: 1
    }
    42%,
    43%,
    46%,
    46.9%,
    52%,
    52.8% {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-3".to_string(),
        r#"@keyframes flicker-3 {
    0%,
    32.98%,
    34.02%,
    34.98%,
    35.92%,
    38.98%,
    39.82%,
    83.98%,
    84.92%,
    to {
        opacity: 1
    }
    33%,
    34%,
    35%,
    35.9%,
    39%,
    39.8%,
    84%,
    84.9% {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-3;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-4".to_string(),
        r#"@keyframes flicker-4 {
    0%,
    31.98%,
    32.82%,
    34.98%,
    35.72%,
    36.98%,
    37.62%,
    67.98%,
    68.42%,
    95.98%,
    96.72%,
    98.98%,
    99.62%,
    to {
        opacity: 1
    }
    32%,
    32.8%,
    35%,
    35.7%,
    37%,
    37.6%,
    68%,
    68.4%,
    96%,
    96.7%,
    99%,
    99.6% {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-4;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-5".to_string(),
        r#"@keyframes flicker-5 {
    -.02%,
    0%,
    1%,
    1.02%,
    101%,
    101.02%,
    14.98%,
    15.18%,
    15.48%,
    15.82%,
    16.02%,
    16.22%,
    16.98%,
    17%,
    17.8%,
    17.82%,
    20.48%,
    20.98%,
    21%,
    21.32%,
    22%,
    22.02%,
    39.98%,
    40%,
    40.48%,
    41%,
    41.02%,
    41.42%,
    41.98%,
    42%,
    42.8%,
    42.82%,
    59.98%,
    60%,
    60.18%,
    60.78%,
    61.02%,
    61.38%,
    61.62%,
    61.78%,
    61.8%,
    62.22%,
    62.8%,
    62.82%,
    75.98%,
    76%,
    77%,
    77.02%,
    77.98%,
    78.82%,
    78.98%,
    79%,
    8.98%,
    80%,
    80.02%,
    9.48%,
    9.5%,
    9.6%,
    9.62%,
    9.82%,
    99.98%,
    to {
        opacity: 1
    }
    61.4%,
    62.2%,
    9%,
    9.8% {
        opacity: 0
    }
    15%,
    15.5%,
    15.8%,
    16.2% {
        opacity: .5
    }
    15.2%,
    16%,
    78%,
    78.8% {
        opacity: .7
    }
    20.5%,
    21.3% {
        opacity: .9
    }
    40.5%,
    41.4% {
        opacity: .6
    }
    60.2%,
    61% {
        opacity: .2
    }
    60.8%,
    61.6% {
        opacity: .4
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-5;
}
"#
        .to_string(),
    );
    map.insert(
        "flicker-in-1".to_string(),
        r#"@keyframes flicker-in-1 {
    0%,
    10%,
    10.2%,
    20%,
    20.6%,
    30%,
    30.6%,
    45%,
    55.1%,
    57%,
    60.1%,
    65%,
    75.1%,
    77%,
    85.1%,
    86% {
        opacity: 0
    }
    10.1%,
    20.1%,
    30.1%,
    30.5%,
    45.1%,
    50%,
    55%,
    57.1%,
    60%,
    65.1%,
    75%,
    77.1%,
    85%,
    86.1%,
    to {
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-in-1;
}
"#
        .to_string(),
    );
    map.insert("flicker-in-2".to_string(), r#"@keyframes flicker-in-2 {
    0% {
        opacity: 0
    }
    10%,
    10.2%,
    20%,
    20.6%,
    30%,
    30.6%,
    45%,
    55.1%,
    57%,
    60.1%,
    65%,
    75.1%,
    77%,
    85.1%,
    86% {
        opacity: 0;
        box-shadow: none
    }
    10.1% {
        opacity: 1;
        box-shadow: none
    }
    20.1% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .25)
    }
    30.1%,
    30.5%,
    45.1%,
    50%,
    55% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .45), 0 0 60px rgba(255, 255, 255, .25)
    }
    57.1%,
    60% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .3)
    }
    65.1%,
    75% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .3), 0 0 100px rgba(255, 255, 255, .1)
    }
    77.1%,
    85% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .6), 0 0 60px rgba(255, 255, 255, .4), 0 0 110px rgba(255, 255, 255, .2), 0 0 100px rgba(255, 255, 255, .1)
    }
    86.1%,
    to {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .6), 0 0 60px rgba(255, 255, 255, .45), 0 0 110px rgba(255, 255, 255, .25), 0 0 100px rgba(255, 255, 255, .1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-in-2;
}
"#.to_string());
    map.insert(
        "flicker-out-1".to_string(),
        r#"@keyframes flicker-out-1 {
    0%,
    13.9%,
    15%,
    22.9%,
    25%,
    34.9%,
    40%,
    42.9%,
    45%,
    50%,
    54.9%,
    69.5%,
    69.9%,
    79.9% {
        opacity: 1
    }
    14%,
    14.9%,
    23%,
    24.9%,
    35%,
    39.9%,
    43%,
    44.9%,
    55%,
    69.4%,
    70%,
    79.4%,
    80%,
    89.8%,
    90% {
        opacity: 0;
        box-shadow: none
    }
    89.9% {
        opacity: 1;
        box-shadow: none
    }
    to {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-out-1;
}
"#
        .to_string(),
    );
    map.insert("flicker-out-2".to_string(), r#"@keyframes flicker-out-2 {
    0%,
    13.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .6), 0 0 60px rgba(255, 255, 255, .45), 0 0 110px rgba(255, 255, 255, .25), 0 0 100px rgba(255, 255, 255, .1)
    }
    14%,
    14.9%,
    23%,
    24.9%,
    35%,
    39.9%,
    43%,
    44.9%,
    55%,
    69.4%,
    70%,
    79.4%,
    80%,
    89.8%,
    90% {
        opacity: 0;
        box-shadow: none
    }
    15%,
    22.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .4), 0 0 110px rgba(255, 255, 255, .2), 0 0 100px rgba(255, 255, 255, .1)
    }
    25%,
    34.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35), 0 0 100px rgba(255, 255, 255, .1)
    }
    40%,
    42.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35)
    }
    45%,
    50%,
    54.9%,
    69.5%,
    69.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .45), 0 0 60px rgba(255, 255, 255, .25)
    }
    79.9% {
        opacity: 1;
        box-shadow: 0 0 30px rgba(255, 255, 255, .25)
    }
    89.9% {
        opacity: 1;
        box-shadow: none
    }
    to {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flicker-out-2;
}
"#.to_string());
    map.insert(
        "flip-2-hor-bottom-1".to_string(),
        r#"@keyframes flip-2-hor-bottom-1 {
    0% {
        transform: translateY(0) rotateX(0);
        transform-origin: 50% 100%
    }
    to {
        transform: translateY(100%) rotateX(180deg);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-bottom-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-bottom-2".to_string(),
        r#"@keyframes flip-2-hor-bottom-2 {
    0% {
        transform: translateY(0) rotateX(0);
        transform-origin: 50% 100%
    }
    to {
        transform: translateY(100%) rotateX(-180deg);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-bottom-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-bottom-bck".to_string(),
        r#"@keyframes flip-2-hor-bottom-bck {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0);
        transform-origin: 50% 100%
    }
    to {
        transform: translateY(100%) translateZ(-260px) rotateX(-180deg);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-bottom-fwd".to_string(),
        r#"@keyframes flip-2-hor-bottom-fwd {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0);
        transform-origin: 50% 100%
    }
    to {
        transform: translateY(100%) translateZ(160px) rotateX(180deg);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-top-1".to_string(),
        r#"@keyframes flip-2-hor-top-1 {
    0% {
        transform: translateY(0) rotateX(0);
        transform-origin: 50% 0%
    }
    to {
        transform: translateY(-100%) rotateX(-180deg);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-top-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-top-2".to_string(),
        r#"@keyframes flip-2-hor-top-2 {
    0% {
        transform: translateY(0) rotateX(0);
        transform-origin: 50% 0%
    }
    to {
        transform: translateY(-100%) rotateX(180deg);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-top-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-top-bck".to_string(),
        r#"@keyframes flip-2-hor-top-bck {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0);
        transform-origin: 50% 0%
    }
    to {
        transform: translateY(-100%) translateZ(-260px) rotateX(180deg);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-hor-top-fwd".to_string(),
        r#"@keyframes flip-2-hor-top-fwd {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0);
        transform-origin: 50% 0%
    }
    to {
        transform: translateY(-100%) translateZ(160px) rotateX(-180deg);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-hor-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-left-1".to_string(),
        r#"@keyframes flip-2-ver-left-1 {
    0% {
        transform: translateX(0) rotateY(0);
        transform-origin: 0% 50%
    }
    to {
        transform: translateX(-100%) rotateY(180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-left-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-left-2".to_string(),
        r#"@keyframes flip-2-ver-left-2 {
    0% {
        transform: translateX(0) rotateY(0);
        transform-origin: 0% 50%
    }
    to {
        transform: translateX(-100%) rotateY(-180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-left-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-left-bck".to_string(),
        r#"@keyframes flip-2-ver-left-bck {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: 0% 50%
    }
    to {
        transform: translateX(-100%) translateZ(-260px) rotateY(-180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-left-fwd".to_string(),
        r#"@keyframes flip-2-ver-left-fwd {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: 0% 50%
    }
    to {
        transform: translateX(-100%) translateZ(160px) rotateY(180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-right-1".to_string(),
        r#"@keyframes flip-2-ver-right-1 {
    0% {
        transform: translateX(0) rotateY(0);
        transform-origin: 100% 50%
    }
    to {
        transform: translateX(100%) rotateY(-180deg);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-right-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-right-2".to_string(),
        r#"@keyframes flip-2-ver-right-2 {
    0% {
        transform: translateX(0) rotateY(0);
        transform-origin: 100% 50%
    }
    to {
        transform: translateX(100%) rotateY(180deg);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-right-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-right-bck".to_string(),
        r#"@keyframes flip-2-ver-right-bck {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: 100% 50%
    }
    to {
        transform: translateX(100%) translateZ(-260px) rotateY(180deg);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-2-ver-right-fwd".to_string(),
        r#"@keyframes flip-2-ver-right-fwd {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: 100% 50%
    }
    to {
        transform: translateX(100%) translateZ(160px) rotateY(-180deg);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-2-ver-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-1-bck".to_string(),
        r#"@keyframes flip-diagonal-1-bck {
    0% {
        transform: translateZ(0) rotate3d(1, 1, 0, 0deg)
    }
    to {
        transform: translateZ(-260px) rotate3d(1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-1-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-1-bl".to_string(),
        r#"@keyframes flip-diagonal-1-bl {
    0% {
        transform: rotate3d(1, 1, 0, 0deg)
    }
    to {
        transform: rotate3d(1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-1-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-1-fwd".to_string(),
        r#"@keyframes flip-diagonal-1-fwd {
    0% {
        transform: translateZ(0) rotate3d(1, 1, 0, 0deg)
    }
    to {
        transform: translateZ(160px) rotate3d(1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-1-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-1-tr".to_string(),
        r#"@keyframes flip-diagonal-1-tr {
    0% {
        transform: rotate3d(1, 1, 0, 0deg)
    }
    to {
        transform: rotate3d(1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-1-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-2-bck".to_string(),
        r#"@keyframes flip-diagonal-2-bck {
    0% {
        transform: translateZ(0) rotate3d(-1, 1, 0, 0deg)
    }
    to {
        transform: translateZ(-260px) rotate3d(-1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-2-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-2-br".to_string(),
        r#"@keyframes flip-diagonal-2-br {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg)
    }
    to {
        transform: rotate3d(-1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-2-br;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-2-fwd".to_string(),
        r#"@keyframes flip-diagonal-2-fwd {
    0% {
        transform: translateZ(0) rotate3d(-1, 1, 0, 0deg)
    }
    to {
        transform: translateZ(160px) rotate3d(-1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-2-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-diagonal-2-tl".to_string(),
        r#"@keyframes flip-diagonal-2-tl {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg)
    }
    to {
        transform: rotate3d(-1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-diagonal-2-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-horizontal-bck".to_string(),
        r#"@keyframes flip-horizontal-bck {
    0% {
        transform: translateZ(0) rotateX(0)
    }
    to {
        transform: translateZ(-260px) rotateX(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-horizontal-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-horizontal-bottom".to_string(),
        r#"@keyframes flip-horizontal-bottom {
    0% {
        transform: rotateX(0)
    }
    to {
        transform: rotateX(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-horizontal-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-horizontal-fwd".to_string(),
        r#"@keyframes flip-horizontal-fwd {
    0% {
        transform: translateZ(0) rotateX(0)
    }
    to {
        transform: translateZ(160px) rotateX(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-horizontal-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-horizontal-top".to_string(),
        r#"@keyframes flip-horizontal-top {
    0% {
        transform: rotateX(0)
    }
    to {
        transform: rotateX(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-horizontal-top;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-diag-1-bl".to_string(),
        r#"@keyframes flip-in-diag-1-bl {
    0% {
        transform: rotate3d(1, 1, 0, 80deg);
        opacity: 0
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-diag-1-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-diag-1-tr".to_string(),
        r#"@keyframes flip-in-diag-1-tr {
    0% {
        transform: rotate3d(1, 1, 0, -80deg);
        opacity: 0
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-diag-1-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-diag-2-br".to_string(),
        r#"@keyframes flip-in-diag-2-br {
    0% {
        transform: rotate3d(-1, 1, 0, -80deg);
        opacity: 0
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-diag-2-br;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-diag-2-tl".to_string(),
        r#"@keyframes flip-in-diag-2-tl {
    0% {
        transform: rotate3d(-1, 1, 0, 80deg);
        opacity: 0
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-diag-2-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-hor-bottom".to_string(),
        r#"@keyframes flip-in-hor-bottom {
    0% {
        transform: rotateX(80deg);
        opacity: 0
    }
    to {
        transform: rotateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-hor-top".to_string(),
        r#"@keyframes flip-in-hor-top {
    0% {
        transform: rotateX(-80deg);
        opacity: 0
    }
    to {
        transform: rotateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-ver-left".to_string(),
        r#"@keyframes flip-in-ver-left {
    0% {
        transform: rotateY(80deg);
        opacity: 0
    }
    to {
        transform: rotateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-ver-left;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-ver-right".to_string(),
        r#"@keyframes flip-in-ver-right {
    0% {
        transform: rotateY(-80deg);
        opacity: 0
    }
    to {
        transform: rotateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-in-ver-right;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-x".to_string(),
        r#"@keyframes flip-in-x {
  from {
    transform: perspective(400px) rotate3d(1, 0, 0, 90deg);
    animation-timing-function: ease-in;
    opacity: 0;
  }

  40% {
    transform: perspective(400px) rotate3d(1, 0, 0, -20deg);
    animation-timing-function: ease-in;
  }

  60% {
    transform: perspective(400px) rotate3d(1, 0, 0, 10deg);
    opacity: 1;
  }

  80% {
    transform: perspective(400px) rotate3d(1, 0, 0, -5deg);
  }

  to {
    transform: perspective(400px);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  backface-visibility: visible !important;
  animation-name: flip-in-x;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-in-y".to_string(),
        r#"@keyframes flip-in-y {
  from {
    transform: perspective(400px) rotate3d(0, 1, 0, 90deg);
    animation-timing-function: ease-in;
    opacity: 0;
  }

  40% {
    transform: perspective(400px) rotate3d(0, 1, 0, -20deg);
    animation-timing-function: ease-in;
  }

  60% {
    transform: perspective(400px) rotate3d(0, 1, 0, 10deg);
    opacity: 1;
  }

  80% {
    transform: perspective(400px) rotate3d(0, 1, 0, -5deg);
  }

  to {
    transform: perspective(400px);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  backface-visibility: visible !important;
  animation-name: flip-in-y;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-diag-1-bl".to_string(),
        r#"@keyframes flip-out-diag-1-bl {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
    to {
        transform: rotate3d(1, 1, 0, -70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-diag-1-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-diag-1-tr".to_string(),
        r#"@keyframes flip-out-diag-1-tr {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
    to {
        transform: rotate3d(1, 1, 0, 70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-diag-1-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-diag-2-br".to_string(),
        r#"@keyframes flip-out-diag-2-br {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
    to {
        transform: rotate3d(-1, 1, 0, 70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-diag-2-br;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-diag-2-tl".to_string(),
        r#"@keyframes flip-out-diag-2-tl {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
    to {
        transform: rotate3d(-1, 1, 0, -70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-diag-2-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-hor-bottom".to_string(),
        r#"@keyframes flip-out-hor-bottom {
    0% {
        transform: rotateX(0);
        opacity: 1
    }
    to {
        transform: rotateX(-70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-hor-top".to_string(),
        r#"@keyframes flip-out-hor-top {
    0% {
        transform: rotateX(0);
        opacity: 1
    }
    to {
        transform: rotateX(70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-ver-left".to_string(),
        r#"@keyframes flip-out-ver-left {
    0% {
        transform: rotateY(0);
        opacity: 1
    }
    to {
        transform: rotateY(-70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-ver-left;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-ver-right".to_string(),
        r#"@keyframes flip-out-ver-right {
    0% {
        transform: rotateY(0);
        opacity: 1
    }
    to {
        transform: rotateY(70deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-out-ver-right;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-x".to_string(),
        r#"@keyframes flip-out-x {
  from {
    transform: perspective(400px);
  }

  30% {
    transform: perspective(400px) rotate3d(1, 0, 0, -20deg);
    opacity: 1;
  }

  to {
    transform: perspective(400px) rotate3d(1, 0, 0, 90deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-duration: 0.75s;
  animation-name: flip-out-x;
  backface-visibility: visible !important;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-out-y".to_string(),
        r#"@keyframes flip-out-y {
  from {
    transform: perspective(400px);
  }

  30% {
    transform: perspective(400px) rotate3d(0, 1, 0, -15deg);
    opacity: 1;
  }

  to {
    transform: perspective(400px) rotate3d(0, 1, 0, 90deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-duration: 0.75s;
  backface-visibility: visible !important;
  animation-name: flip-out-y;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-2-hor-bottom".to_string(),
        r#"@keyframes flip-scale-2-hor-bottom {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 100%
    }
    50% {
        transform: translateY(50%) rotateX(90deg) scale(2);
        transform-origin: 50% 50%
    }
    to {
        transform: translateY(100%) rotateX(180deg) scale(1);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-2-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-2-hor-top".to_string(),
        r#"@keyframes flip-scale-2-hor-top {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 0%
    }
    50% {
        transform: translateY(-50%) rotateX(-90deg) scale(2);
        transform-origin: 50% 50%
    }
    to {
        transform: translateY(-100%) rotateX(-180deg) scale(1);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-2-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-2-ver-left".to_string(),
        r#"@keyframes flip-scale-2-ver-left {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 0% 50%
    }
    50% {
        transform: translateX(-50%) rotateY(90deg) scale(2);
        transform-origin: 50% 50%
    }
    to {
        transform: translateX(-100%) rotateY(180deg) scale(1);
        transform-origin: 100% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-2-ver-left;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-2-ver-right".to_string(),
        r#"@keyframes flip-scale-2-ver-right {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 100% 50%
    }
    50% {
        transform: translateX(50%) rotateY(-90deg) scale(2);
        transform-origin: 50% 50%
    }
    to {
        transform: translateX(100%) rotateY(-180deg) scale(1);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-2-ver-right;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-down-diag-1".to_string(),
        r#"@keyframes flip-scale-down-diag-1 {
    0% {
        transform: scale(1) rotate3d(1, 1, 0, 0deg)
    }
    50% {
        transform: scale(.4) rotate3d(1, 1, 0, -90deg)
    }
    to {
        transform: scale(1) rotate3d(1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-down-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-down-diag-2".to_string(),
        r#"@keyframes flip-scale-down-diag-2 {
    0% {
        transform: scale(1) rotate3d(-1, 1, 0, 0deg)
    }
    50% {
        transform: scale(.4) rotate3d(-1, 1, 0, -90deg)
    }
    to {
        transform: scale(1) rotate3d(-1, 1, 0, -180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-down-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-down-hor".to_string(),
        r#"@keyframes flip-scale-down-hor {
    0% {
        transform: scale(1) rotateX(0)
    }
    50% {
        transform: scale(.4) rotateX(90deg)
    }
    to {
        transform: scale(1) rotateX(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-down-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-down-ver".to_string(),
        r#"@keyframes flip-scale-down-ver {
    0% {
        transform: scale(1) rotateY(0)
    }
    50% {
        transform: scale(.4) rotateY(-90deg)
    }
    to {
        transform: scale(1) rotateY(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-down-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-up-diag-1".to_string(),
        r#"@keyframes flip-scale-up-diag-1 {
    0% {
        transform: scale(1) rotate3d(1, 1, 0, 0deg)
    }
    50% {
        transform: scale(2.5) rotate3d(1, 1, 0, 90deg)
    }
    to {
        transform: scale(1) rotate3d(1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-up-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-up-diag-2".to_string(),
        r#"@keyframes flip-scale-up-diag-2 {
    0% {
        transform: scale(1) rotate3d(-1, 1, 0, 0deg)
    }
    50% {
        transform: scale(2.5) rotate3d(-1, 1, 0, 90deg)
    }
    to {
        transform: scale(1) rotate3d(-1, 1, 0, 180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-up-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-up-hor".to_string(),
        r#"@keyframes flip-scale-up-hor {
    0% {
        transform: scale(1) rotateX(0)
    }
    50% {
        transform: scale(2.5) rotateX(-90deg)
    }
    to {
        transform: scale(1) rotateX(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-up-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-scale-up-ver".to_string(),
        r#"@keyframes flip-scale-up-ver {
    0% {
        transform: scale(1) rotateY(0)
    }
    50% {
        transform: scale(2.5) rotateY(90deg)
    }
    to {
        transform: scale(1) rotateY(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-scale-up-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-vertical-bck".to_string(),
        r#"@keyframes flip-vertical-bck {
    0% {
        transform: translateZ(0) rotateY(0)
    }
    to {
        transform: translateZ(-260px) rotateY(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-vertical-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-vertical-fwd".to_string(),
        r#"@keyframes flip-vertical-fwd {
    0% {
        transform: translateZ(0) rotateY(0)
    }
    to {
        transform: translateZ(160px) rotateY(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-vertical-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-vertical-left".to_string(),
        r#"@keyframes flip-vertical-left {
    0% {
        transform: rotateY(0)
    }
    to {
        transform: rotateY(-180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-vertical-left;
}
"#
        .to_string(),
    );
    map.insert(
        "flip-vertical-right".to_string(),
        r#"@keyframes flip-vertical-right {
    0% {
        transform: rotateY(0)
    }
    to {
        transform: rotateY(180deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: flip-vertical-right;
}
"#
        .to_string(),
    );
    map.insert(
        "flip".to_string(),
        r#"@keyframes flip {
  from {
    transform: perspective(400px) scale3d(1, 1, 1) translate3d(0, 0, 0) rotate3d(0, 1, 0, -360deg);
    animation-timing-function: ease-out;
  }

  40% {
    transform: perspective(400px) scale3d(1, 1, 1) translate3d(0, 0, 150px)
      rotate3d(0, 1, 0, -190deg);
    animation-timing-function: ease-out;
  }

  50% {
    transform: perspective(400px) scale3d(1, 1, 1) translate3d(0, 0, 150px)
      rotate3d(0, 1, 0, -170deg);
    animation-timing-function: ease-in;
  }

  80% {
    transform: perspective(400px) scale3d(0.95, 0.95, 0.95) translate3d(0, 0, 0)
      rotate3d(0, 1, 0, 0deg);
    animation-timing-function: ease-in;
  }

  to {
    transform: perspective(400px) scale3d(1, 1, 1) translate3d(0, 0, 0) rotate3d(0, 1, 0, 0deg);
    animation-timing-function: ease-in;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  backface-visibility: visible;
  animation-name: flip;
}
"#
        .to_string(),
    );
    map.insert(
        "focus-in-contract-bck".to_string(),
        r#"@keyframes focus-in-contract-bck {
    0% {
        letter-spacing: 1em;
        transform: translateZ(300px);
        filter: blur(12px);
        opacity: 0
    }
    to {
        transform: translateZ(12px);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: focus-in-contract-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "focus-in-contract".to_string(),
        r#"@keyframes focus-in-contract {
    0% {
        letter-spacing: 1em;
        filter: blur(12px);
        opacity: 0
    }
    to {
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: focus-in-contract;
}
"#
        .to_string(),
    );
    map.insert(
        "focus-in-expand-fwd".to_string(),
        r#"@keyframes focus-in-expand-fwd {
    0% {
        letter-spacing: -.5em;
        transform: translateZ(-800px);
        filter: blur(12px);
        opacity: 0
    }
    to {
        transform: translateZ(0);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: focus-in-expand-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "focus-in-expand".to_string(),
        r#"@keyframes focus-in-expand {
    0% {
        letter-spacing: -.5em;
        filter: blur(12px);
        opacity: 0
    }
    to {
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: focus-in-expand;
}
"#
        .to_string(),
    );
    map.insert(
        "head-shake".to_string(),
        r#"@keyframes head-shake {
  0% {
    transform: translateX(0);
  }

  6.5% {
    transform: translateX(-6px) rotateY(-9deg);
  }

  18.5% {
    transform: translateX(5px) rotateY(7deg);
  }

  31.5% {
    transform: translateX(-3px) rotateY(-5deg);
  }

  43.5% {
    transform: translateX(2px) rotateY(3deg);
  }

  50% {
    transform: translateX(0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-timing-function: ease-in-out;
  animation-name: head-shake;
}
"#
        .to_string(),
    );
    map.insert(
        "heart-beat".to_string(),
        r#"@keyframes heart-beat {
  0% {
    transform: scale(1);
  }

  14% {
    transform: scale(1.3);
  }

  28% {
    transform: scale(1);
  }

  42% {
    transform: scale(1.3);
  }

  70% {
    transform: scale(1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: heart-beat;
  animation-duration: 1.3s;
  animation-timing-function: ease-in-out;
}
"#
        .to_string(),
    );
    map.insert(
        "heartbeat".to_string(),
        r#"@keyframes heartbeat {
    0% {
        transform: scale(1);
        transform-origin: center center;
        animation-timing-function: ease-out
    }
    10% {
        transform: scale(.91);
        animation-timing-function: ease-in
    }
    17% {
        transform: scale(.98);
        animation-timing-function: ease-out
    }
    33% {
        transform: scale(.87);
        animation-timing-function: ease-in
    }
    45% {
        transform: scale(1);
        animation-timing-function: ease-out
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: heartbeat;
}
"#
        .to_string(),
    );
    map.insert(
        "hinge".to_string(),
        r#"@keyframes hinge {
  0% {
    animation-timing-function: ease-in-out;
  }

  20%,
  60% {
    transform: rotate3d(0, 0, 1, 80deg);
    animation-timing-function: ease-in-out;
  }

  40%,
  80% {
    transform: rotate3d(0, 0, 1, 60deg);
    animation-timing-function: ease-in-out;
    opacity: 1;
  }

  to {
    transform: translate3d(0, 700px, 0);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-duration: 2s;
  animation-name: hinge;
  transform-origin: top left;
}
"#
        .to_string(),
    );
    map.insert(
        "jack-in-the-box".to_string(),
        r#"@keyframes jack-in-the-box {
  from {
    opacity: 0;
    transform: scale(0.1) rotate(30deg);
    transform-origin: center bottom;
  }

  50% {
    transform: rotate(-10deg);
  }

  70% {
    transform: rotate(3deg);
  }

  to {
    opacity: 1;
    transform: scale(1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jack-in-the-box;
}
"#
        .to_string(),
    );
    map.insert(
        "jello-diagonal-1".to_string(),
        r#"@keyframes jello-diagonal-1 {
    0%,
    to {
        transform: skew(0deg 0deg)
    }
    30% {
        transform: skew(25deg 25deg)
    }
    40% {
        transform: skew(-15deg, -15deg)
    }
    50% {
        transform: skew(15deg, 15deg)
    }
    65% {
        transform: skew(-5deg, -5deg)
    }
    75% {
        transform: skew(5deg, 5deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jello-diagonal-1;
}
"#
        .to_string(),
    );
    map.insert(
        "jello-diagonal-2".to_string(),
        r#"@keyframes jello-diagonal-2 {
    0%,
    to {
        transform: skew(0deg 0deg)
    }
    30% {
        transform: skew(-25deg -25deg)
    }
    40% {
        transform: skew(15deg, 15deg)
    }
    50% {
        transform: skew(-15deg, -15deg)
    }
    65% {
        transform: skew(5deg, 5deg)
    }
    75% {
        transform: skew(-5deg, -5deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jello-diagonal-2;
}
"#
        .to_string(),
    );
    map.insert(
        "jello-horizontal".to_string(),
        r#"@keyframes jello-horizontal {
    0%,
    to {
        transform: scale3d(1, 1, 1)
    }
    30% {
        transform: scale3d(1.25, .75, 1)
    }
    40% {
        transform: scale3d(.75, 1.25, 1)
    }
    50% {
        transform: scale3d(1.15, .85, 1)
    }
    65% {
        transform: scale3d(.95, 1.05, 1)
    }
    75% {
        transform: scale3d(1.05, .95, 1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jello-horizontal;
}
"#
        .to_string(),
    );
    map.insert(
        "jello-vertical".to_string(),
        r#"@keyframes jello-vertical {
    0%,
    to {
        transform: scale3d(1, 1, 1)
    }
    30% {
        transform: scale3d(.75, 1.25, 1)
    }
    40% {
        transform: scale3d(1.25, .75, 1)
    }
    50% {
        transform: scale3d(.85, 1.15, 1)
    }
    65% {
        transform: scale3d(1.05, .95, 1)
    }
    75% {
        transform: scale3d(.95, 1.05, 1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jello-vertical;
}
"#
        .to_string(),
    );
    map.insert(
        "jello".to_string(),
        r#"@keyframes jello {
  from,
  11.1%,
  to {
    transform: translate3d(0, 0, 0);
  }

  22.2% {
    transform: skewX(-12.5deg) skewY(-12.5deg);
  }

  33.3% {
    transform: skewX(6.25deg) skewY(6.25deg);
  }

  44.4% {
    transform: skewX(-3.125deg) skewY(-3.125deg);
  }

  55.5% {
    transform: skewX(1.5625deg) skewY(1.5625deg);
  }

  66.6% {
    transform: skewX(-0.78125deg) skewY(-0.78125deg);
  }

  77.7% {
    transform: skewX(0.390625deg) skewY(0.390625deg);
  }

  88.8% {
    transform: skewX(-0.1953125deg) skewY(-0.1953125deg);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: jello;
  transform-origin: center;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-bottom-left".to_string(),
        r#"@keyframes kenburns-bottom-left {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 16% 84%
    }
    to {
        transform: scale(1.25) translate(-20px, 15px);
        transform-origin: left bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-bottom-left;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-bottom-right".to_string(),
        r#"@keyframes kenburns-bottom-right {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 84% 84%
    }
    to {
        transform: scale(1.25) translate(20px, 15px);
        transform-origin: right bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-bottom-right;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-bottom".to_string(),
        r#"@keyframes kenburns-bottom {
    0% {
        transform: scale(1) translateY(0);
        transform-origin: 50% 84%
    }
    to {
        transform: scale(1.25) translateY(15px);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-left".to_string(),
        r#"@keyframes kenburns-left {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 16% 50%
    }
    to {
        transform: scale(1.25) translate(-20px, 15px);
        transform-origin: left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-left;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-right".to_string(),
        r#"@keyframes kenburns-right {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 84% 50%
    }
    to {
        transform: scale(1.25) translateX(20px);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-right;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-top-left".to_string(),
        r#"@keyframes kenburns-top-left {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 16% 16%
    }
    to {
        transform: scale(1.25) translate(-20px, -15px);
        transform-origin: top left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-top-left;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-top-right".to_string(),
        r#"@keyframes kenburns-top-right {
    0% {
        transform: scale(1) translate(0, 0);
        transform-origin: 84% 16%
    }
    to {
        transform: scale(1.25) translate(20px, -15px);
        transform-origin: right top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-top-right;
}
"#
        .to_string(),
    );
    map.insert(
        "kenburns-top".to_string(),
        r#"@keyframes kenburns-top {
    0% {
        transform: scale(1) translateY(0);
        transform-origin: 50% 16%
    }
    to {
        transform: scale(1.25) translateY(-15px);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: kenburns-top;
}
"#
        .to_string(),
    );
    map.insert(
        "light-speed-in-left".to_string(),
        r#"@keyframes light-speed-in-left {
  from {
    transform: translate3d(-100%, 0, 0) skewX(30deg);
    opacity: 0;
  }

  60% {
    transform: skewX(-20deg);
    opacity: 1;
  }

  80% {
    transform: skewX(5deg);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: light-speed-in-left;
  animation-timing-function: ease-out;
}
"#
        .to_string(),
    );
    map.insert(
        "light-speed-in-right".to_string(),
        r#"@keyframes light-speed-in-right {
  from {
    transform: translate3d(100%, 0, 0) skewX(-30deg);
    opacity: 0;
  }

  60% {
    transform: skewX(20deg);
    opacity: 1;
  }

  80% {
    transform: skewX(-5deg);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: light-speed-in-right;
  animation-timing-function: ease-out;
}
"#
        .to_string(),
    );
    map.insert(
        "light-speed-out-left".to_string(),
        r#"@keyframes light-speed-out-left {
  from {
    opacity: 1;
  }

  to {
    transform: translate3d(-100%, 0, 0) skewX(-30deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: light-speed-out-left;
  animation-timing-function: ease-in;
}
"#
        .to_string(),
    );
    map.insert(
        "light-speed-out-right".to_string(),
        r#"@keyframes light-speed-out-right {
  from {
    opacity: 1;
  }

  to {
    transform: translate3d(100%, 0, 0) skewX(30deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: light-speed-out-right;
  animation-timing-function: ease-in;
}
"#
        .to_string(),
    );
    map.insert(
        "ping".to_string(),
        r#"@keyframes ping {
    0% {
        transform: scale(.2);
        opacity: .8
    }
    80% {
        transform: scale(1.2);
        opacity: 0
    }
    to {
        transform: scale(2.2);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: ping;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-bl".to_string(),
        r#"@keyframes puff-in-bl {
    0% {
        transform: scale(2);
        transform-origin: 0% 100%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 0% 100%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-bottom".to_string(),
        r#"@keyframes puff-in-bottom {
    0% {
        transform: scale(2);
        transform-origin: 50% 100%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 50% 100%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-br".to_string(),
        r#"@keyframes puff-in-br {
    0% {
        transform: scale(2);
        transform-origin: 100% 100%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 100% 100%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-br;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-center".to_string(),
        r#"@keyframes puff-in-center {
    0% {
        transform: scale(2);
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-center;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-hor".to_string(),
        r#"@keyframes puff-in-hor {
    0% {
        transform: scaleX(2);
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scaleX(1);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-left".to_string(),
        r#"@keyframes puff-in-left {
    0% {
        transform: scale(2);
        transform-origin: 0% 50%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 0% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-right".to_string(),
        r#"@keyframes puff-in-right {
    0% {
        transform: scale(2);
        transform-origin: 100% 50%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 100% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-tl".to_string(),
        r#"@keyframes puff-in-tl {
    0% {
        transform: scale(2);
        transform-origin: 0% 0%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 0% 0%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-top".to_string(),
        r#"@keyframes puff-in-top {
    0% {
        transform: scale(2);
        transform-origin: 50% 0%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 50% 0%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-tr".to_string(),
        r#"@keyframes puff-in-tr {
    0% {
        transform: scale(2);
        transform-origin: 100% 0%;
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scale(1);
        transform-origin: 100% 0%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-in-ver".to_string(),
        r#"@keyframes puff-in-ver {
    0% {
        transform: scaleY(2);
        filter: blur(2px);
        opacity: 0
    }
    to {
        transform: scaleY(1);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-in-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-bl".to_string(),
        r#"@keyframes puff-out-bl {
    0% {
        transform: scale(1);
        transform-origin: 0% 100%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 0% 100%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-bottom".to_string(),
        r#"@keyframes puff-out-bottom {
    0% {
        transform: scale(1);
        transform-origin: 50% 100%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 50% 100%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-br".to_string(),
        r#"@keyframes puff-out-br {
    0% {
        transform: scale(1);
        transform-origin: 100% 100%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 100% 100%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-br;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-center".to_string(),
        r#"@keyframes puff-out-center {
    0% {
        transform: scale(1);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-center;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-hor".to_string(),
        r#"@keyframes puff-out-hor {
    0% {
        transform: scaleX(1);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scaleX(2);
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-left".to_string(),
        r#"@keyframes puff-out-left {
    0% {
        transform: scale(1);
        transform-origin: 0% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 0% 50%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-right".to_string(),
        r#"@keyframes puff-out-right {
    0% {
        transform: scale(1);
        transform-origin: 100% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 100% 50%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-tl".to_string(),
        r#"@keyframes puff-out-tl {
    0% {
        transform: scale(1);
        transform-origin: 0% 0%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 0% 0%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-top".to_string(),
        r#"@keyframes puff-out-top {
    0% {
        transform: scale(1);
        transform-origin: 50% 0%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 50% 0%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-tr".to_string(),
        r#"@keyframes puff-out-tr {
    0% {
        transform: scale(1);
        transform-origin: 100% 0%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scale(2);
        transform-origin: 100% 0%;
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "puff-out-ver".to_string(),
        r#"@keyframes puff-out-ver {
    0% {
        transform: scaleY(1);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: scaleY(2);
        filter: blur(2px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: puff-out-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "pulsate-bck".to_string(),
        r#"@keyframes pulsate-bck {
    0%,
    to {
        transform: scale(1)
    }
    50% {
        transform: scale(.9)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: pulsate-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "pulsate-fwd".to_string(),
        r#"@keyframes pulsate-fwd {
    0%,
    to {
        transform: scale(1)
    }
    50% {
        transform: scale(1.1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: pulsate-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "pulse".to_string(),
        r#"@keyframes pulse {
  from {
    transform: scale3d(1, 1, 1);
  }

  50% {
    transform: scale3d(1.05, 1.05, 1.05);
  }

  to {
    transform: scale3d(1, 1, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: pulse;
  animation-timing-function: ease-in-out;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-blurred-bottom".to_string(),
        r#"@keyframes roll-in-blurred-bottom {
    0% {
        transform: translateY(800px) rotate(720deg);
        filter: blur(50px);
        opacity: 0
    }
    to {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-blurred-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-blurred-left".to_string(),
        r#"@keyframes roll-in-blurred-left {
    0% {
        transform: translateX(-1000px) rotate(-720deg);
        filter: blur(50px);
        opacity: 0
    }
    to {
        transform: translateX(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-blurred-left;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-blurred-right".to_string(),
        r#"@keyframes roll-in-blurred-right {
    0% {
        transform: translateX(1000px) rotate(720deg);
        filter: blur(50px);
        opacity: 0
    }
    to {
        transform: translateX(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-blurred-right;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-blurred-top".to_string(),
        r#"@keyframes roll-in-blurred-top {
    0% {
        transform: translateY(-800px) rotate(-720deg);
        filter: blur(50px);
        opacity: 0
    }
    to {
        transform: translateY(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-blurred-top;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-bottom".to_string(),
        r#"@keyframes roll-in-bottom {
    0% {
        transform: translateY(800px) rotate(540deg);
        opacity: 0
    }
    to {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-left".to_string(),
        r#"@keyframes roll-in-left {
    0% {
        transform: translateX(-800px) rotate(-540deg);
        opacity: 0
    }
    to {
        transform: translateX(0) rotate(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-right".to_string(),
        r#"@keyframes roll-in-right {
    0% {
        transform: translateX(800px) rotate(540deg);
        opacity: 0
    }
    to {
        transform: translateX(0) rotate(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in-top".to_string(),
        r#"@keyframes roll-in-top {
    0% {
        transform: translateY(-800px) rotate(-540deg);
        opacity: 0
    }
    to {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-in".to_string(),
        r#"@keyframes roll-in {
  from {
    opacity: 0;
    transform: translate3d(-100%, 0, 0) rotate3d(0, 0, 1, -120deg);
  }

  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-in;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-blurred-bottom".to_string(),
        r#"@keyframes roll-out-blurred-bottom {
    0% {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
    to {
        transform: translateY(800px) rotate(720deg);
        filter: blur(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-blurred-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-blurred-left".to_string(),
        r#"@keyframes roll-out-blurred-left {
    0% {
        transform: translateX(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateX(-1000px) rotate(-720deg);
        filter: blur(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-blurred-left;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-blurred-right".to_string(),
        r#"@keyframes roll-out-blurred-right {
    0% {
        transform: translateX(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateX(1000px) rotate(720deg);
        filter: blur(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-blurred-right;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-blurred-top".to_string(),
        r#"@keyframes roll-out-blurred-top {
    0% {
        transform: translateY(0) rotate(0deg);
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateY(-800px) rotate(-720deg);
        filter: blur(50px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-blurred-top;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-bottom".to_string(),
        r#"@keyframes roll-out-bottom {
    0% {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
    to {
        transform: translateY(800px) rotate(540deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-left".to_string(),
        r#"@keyframes roll-out-left {
    0% {
        transform: translateX(0) rotate(0deg);
        opacity: 1
    }
    to {
        transform: translateX(-1000px) rotate(-540deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-right".to_string(),
        r#"@keyframes roll-out-right {
    0% {
        transform: translateX(0) rotate(0deg);
        opacity: 1
    }
    to {
        transform: translateX(1000px) rotate(540deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out-top".to_string(),
        r#"@keyframes roll-out-top {
    0% {
        transform: translateY(0) rotate(0deg);
        opacity: 1
    }
    to {
        transform: translateY(-800px) rotate(-540deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "roll-out".to_string(),
        r#"@keyframes roll-out {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
    transform: translate3d(100%, 0, 0) rotate3d(0, 0, 1, 120deg);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: roll-out;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-bl-ccw".to_string(),
        r#"@keyframes rotate-90-bl-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 0% 100%
    }
    to {
        transform: rotate(-90deg);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-bl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-bl-cw".to_string(),
        r#"@keyframes rotate-90-bl-cw {
    0% {
        transform: rotate(0);
        transform-origin: 0% 100%
    }
    to {
        transform: rotate(90deg);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-bl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-bottom-ccw".to_string(),
        r#"@keyframes rotate-90-bottom-ccw {
    0% {
        transform: rotate(0);
        transform-origin: bottom
    }
    to {
        transform: rotate(-90deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-bottom-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-bottom-cw".to_string(),
        r#"@keyframes rotate-90-bottom-cw {
    0% {
        transform: rotate(0);
        transform-origin: bottom
    }
    to {
        transform: rotate(90deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-bottom-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-br-ccw".to_string(),
        r#"@keyframes rotate-90-br-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 100%
    }
    to {
        transform: rotate(-90deg);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-br-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-br-cw".to_string(),
        r#"@keyframes rotate-90-br-cw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 100%
    }
    to {
        transform: rotate(90deg);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-br-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-ccw".to_string(),
        r#"@keyframes rotate-90-ccw {
    0% {
        transform: rotate(0)
    }
    to {
        transform: rotate(-90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-cw".to_string(),
        r#"@keyframes rotate-90-cw {
    0% {
        transform: rotate(0)
    }
    to {
        transform: rotate(90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-horizontal-bck".to_string(),
        r#"@keyframes rotate-90-horizontal-bck {
    0% {
        transform: rotateX(0)
    }
    to {
        transform: rotateX(-90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-horizontal-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-horizontal-fwd".to_string(),
        r#"@keyframes rotate-90-horizontal-fwd {
    0% {
        transform: rotateX(0)
    }
    to {
        transform: rotateX(90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-horizontal-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-left-ccw".to_string(),
        r#"@keyframes rotate-90-left-ccw {
    0% {
        transform: rotate(0);
        transform-origin: left
    }
    to {
        transform: rotate(-90deg);
        transform-origin: left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-left-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-left-cw".to_string(),
        r#"@keyframes rotate-90-left-cw {
    0% {
        transform: rotate(0);
        transform-origin: left
    }
    to {
        transform: rotate(90deg);
        transform-origin: left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-left-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-right-ccw".to_string(),
        r#"@keyframes rotate-90-right-ccw {
    0% {
        transform: rotate(0);
        transform-origin: right
    }
    to {
        transform: rotate(-90deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-right-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-right-cw".to_string(),
        r#"@keyframes rotate-90-right-cw {
    0% {
        transform: rotate(0);
        transform-origin: right
    }
    to {
        transform: rotate(90deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-right-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-tl-ccw".to_string(),
        r#"@keyframes rotate-90-tl-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 0% 0%
    }
    to {
        transform: rotate(-90deg);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-tl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-tl-cw".to_string(),
        r#"@keyframes rotate-90-tl-cw {
    0% {
        transform: rotate(0);
        transform-origin: 0% 0%
    }
    to {
        transform: rotate(90deg);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-tl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-top-ccw".to_string(),
        r#"@keyframes rotate-90-top-ccw {
    0% {
        transform: rotate(0);
        transform-origin: top
    }
    to {
        transform: rotate(-90deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-top-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-top-cw".to_string(),
        r#"@keyframes rotate-90-top-cw {
    0% {
        transform: rotate(0);
        transform-origin: top
    }
    to {
        transform: rotate(90deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-top-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-tr-ccw".to_string(),
        r#"@keyframes rotate-90-tr-ccw {
    0% {
        transform: rotate(0);
        transform-origin: top right
    }
    to {
        transform: rotate(-90deg);
        transform-origin: top right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-tr-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-tr-cw".to_string(),
        r#"@keyframes rotate-90-tr-cw {
    0% {
        transform: rotate(0);
        transform-origin: top right
    }
    to {
        transform: rotate(90deg);
        transform-origin: top right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-tr-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-vertical-bck".to_string(),
        r#"@keyframes rotate-90-vertical-bck {
    0% {
        transform: rotateY(0)
    }
    to {
        transform: rotateY(-90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-vertical-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-90-vertical-fwd".to_string(),
        r#"@keyframes rotate-90-vertical-fwd {
    0% {
        transform: rotateY(0)
    }
    to {
        transform: rotateY(90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-90-vertical-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-bl".to_string(),
        r#"@keyframes rotate-bl {
    0% {
        transform: rotate(0);
        transform-origin: bottom left
    }
    to {
        transform: rotate(360deg);
        transform-origin: bottom left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-bottom".to_string(),
        r#"@keyframes rotate-bottom {
    0% {
        transform: rotate(0);
        transform-origin: bottom
    }
    to {
        transform: rotate(360deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-br".to_string(),
        r#"@keyframes rotate-br {
    0% {
        transform: rotate(0);
        transform-origin: bottom right
    }
    to {
        transform: rotate(360deg);
        transform-origin: bottom right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-br;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-center".to_string(),
        r#"@keyframes rotate-center {
    0% {
        transform: rotate(0)
    }
    to {
        transform: rotate(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-1".to_string(),
        r#"@keyframes rotate-diagonal-1 {
    0% {
        transform: rotate3d(1, 1, 0, 0deg)
    }
    50% {
        transform: rotate3d(1, 1, 0, -180deg)
    }
    to {
        transform: rotate3d(1, 1, 0, -360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-1;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-2".to_string(),
        r#"@keyframes rotate-diagonal-2 {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg)
    }
    50% {
        transform: rotate3d(-1, 1, 0, 180deg)
    }
    to {
        transform: rotate3d(-1, 1, 0, 360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-2;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-bl".to_string(),
        r#"@keyframes rotate-diagonal-bl {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 0% 100%
    }
    50% {
        transform: rotate3d(1, 1, 0, 180deg);
        transform-origin: 0% 100%
    }
    to {
        transform: rotate3d(1, 1, 0, 360deg);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-br".to_string(),
        r#"@keyframes rotate-diagonal-br {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 100% 100%
    }
    50% {
        transform: rotate3d(-1, 1, 0, -180deg);
        transform-origin: 100% 100%
    }
    to {
        transform: rotate3d(-1, 1, 0, -360deg);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-br;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-tl".to_string(),
        r#"@keyframes rotate-diagonal-tl {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 0% 0%
    }
    50% {
        transform: rotate3d(-1, 1, 0, 180deg);
        transform-origin: 0% 0%
    }
    to {
        transform: rotate3d(-1, 1, 0, 360deg);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-diagonal-tr".to_string(),
        r#"@keyframes rotate-diagonal-tr {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 100% 0%
    }
    50% {
        transform: rotate3d(1, 1, 0, -180deg);
        transform-origin: 100% 0%
    }
    to {
        transform: rotate3d(1, 1, 0, -360deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-diagonal-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-hor-bottom".to_string(),
        r#"@keyframes rotate-hor-bottom {
    0% {
        transform: rotateX(0);
        transform-origin: bottom
    }
    to {
        transform: rotateX(360deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-hor-center".to_string(),
        r#"@keyframes rotate-hor-center {
    0% {
        transform: rotateX(0)
    }
    to {
        transform: rotateX(-360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-hor-center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-hor-top".to_string(),
        r#"@keyframes rotate-hor-top {
    0% {
        transform: rotateX(0);
        transform-origin: top
    }
    to {
        transform: rotateX(-360deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-bck".to_string(),
        r#"@keyframes rotate-in-2-bck {
    0% {
        transform: translateZ(200px) rotate(45deg);
        opacity: 0
    }
    to {
        transform: translateZ(0) rotate(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-bl-ccw".to_string(),
        r#"@keyframes rotate-in-2-bl-ccw {
    0% {
        transform: rotate(45deg);
        transform-origin: 0 100%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 0 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-bl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-bl-cw".to_string(),
        r#"@keyframes rotate-in-2-bl-cw {
    0% {
        transform: rotate(-45deg);
        transform-origin: 0 100%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 0 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-bl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-br-ccw".to_string(),
        r#"@keyframes rotate-in-2-br-ccw {
    0% {
        transform: rotate(45deg);
        transform-origin: 100% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-br-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-br-cw".to_string(),
        r#"@keyframes rotate-in-2-br-cw {
    0% {
        transform: rotate(-45deg);
        transform-origin: 100% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-br-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-ccw".to_string(),
        r#"@keyframes rotate-in-2-ccw {
    0% {
        transform: rotate(45deg);
        opacity: 0
    }
    to {
        transform: rotate(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-cw".to_string(),
        r#"@keyframes rotate-in-2-cw {
    0% {
        transform: rotate(-45deg);
        opacity: 0
    }
    to {
        transform: rotate(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-fwd".to_string(),
        r#"@keyframes rotate-in-2-fwd {
    0% {
        transform: translateZ(-200px) rotate(-45deg);
        opacity: 0
    }
    to {
        transform: translateZ(0) rotate(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-tl-ccw".to_string(),
        r#"@keyframes rotate-in-2-tl-ccw {
    0% {
        transform: rotate(45deg);
        transform-origin: 0 0;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-tl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-tl-cw".to_string(),
        r#"@keyframes rotate-in-2-tl-cw {
    0% {
        transform: rotate(-45deg);
        transform-origin: 0 0;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-tl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-tr-ccw".to_string(),
        r#"@keyframes rotate-in-2-tr-ccw {
    0% {
        transform: rotate(45deg);
        transform-origin: 100% 0%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-tr-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-2-tr-cw".to_string(),
        r#"@keyframes rotate-in-2-tr-cw {
    0% {
        transform: rotate(-45deg);
        transform-origin: 100% 0%;
        opacity: 0
    }
    to {
        transform: rotate(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-2-tr-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-bl".to_string(),
        r#"@keyframes rotate-in-bl {
    0% {
        transform: rotate(-360deg);
        transform-origin: bottom left;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: bottom left;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-bottom".to_string(),
        r#"@keyframes rotate-in-bottom {
    0% {
        transform: rotate(-360deg);
        transform-origin: bottom;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: bottom;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-br".to_string(),
        r#"@keyframes rotate-in-br {
    0% {
        transform: rotate(-360deg);
        transform-origin: bottom right;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: bottom right;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-br;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-center".to_string(),
        r#"@keyframes rotate-in-center {
    0% {
        transform: rotate(-360deg);
        opacity: 0
    }
    to {
        transform: rotate(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-diag-1".to_string(),
        r#"@keyframes rotate-in-diag-1 {
    0% {
        transform: rotate3d(1, 1, 0, -360deg);
        opacity: 0
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-diag-2".to_string(),
        r#"@keyframes rotate-in-diag-2 {
    0% {
        transform: rotate3d(-1, 1, 0, -360deg);
        opacity: 0
    }
    to {
        transform: rotate3d(-1, 1, 0, 0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-down-left".to_string(),
        r#"@keyframes rotate-in-down-left {
  from {
    transform: rotate3d(0, 0, 1, -45deg);
    opacity: 0;
  }

  to {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-down-left;
  transform-origin: left bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-down-right".to_string(),
        r#"@keyframes rotate-in-down-right {
  from {
    transform: rotate3d(0, 0, 1, 45deg);
    opacity: 0;
  }

  to {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-down-right;
  transform-origin: right bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-hor".to_string(),
        r#"@keyframes rotate-in-hor {
    0% {
        transform: rotateX(360deg);
        opacity: 0
    }
    to {
        transform: rotateX(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-left".to_string(),
        r#"@keyframes rotate-in-left {
    0% {
        transform: rotate(-360deg);
        transform-origin: left;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: left;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-right".to_string(),
        r#"@keyframes rotate-in-right {
    0% {
        transform: rotate(-360deg);
        transform-origin: right;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: right;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-tl".to_string(),
        r#"@keyframes rotate-in-tl {
    0% {
        transform: rotate(-360deg);
        transform-origin: top left;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: top left;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-top".to_string(),
        r#"@keyframes rotate-in-top {
    0% {
        transform: rotate(-360deg);
        transform-origin: top;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: top;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-tr".to_string(),
        r#"@keyframes rotate-in-tr {
    0% {
        transform: rotate(-360deg);
        transform-origin: top right;
        opacity: 0
    }
    to {
        transform: rotate(0deg);
        transform-origin: top right;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-up-left".to_string(),
        r#"@keyframes rotate-in-up-left {
  from {
    transform: rotate3d(0, 0, 1, 45deg);
    opacity: 0;
  }

  to {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-up-left;
  transform-origin: left bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-up-right".to_string(),
        r#"@keyframes rotate-in-up-right {
  from {
    transform: rotate3d(0, 0, 1, -90deg);
    opacity: 0;
  }

  to {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-up-right;
  transform-origin: right bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in-ver".to_string(),
        r#"@keyframes rotate-in-ver {
    0% {
        transform: rotateY(-360deg);
        opacity: 0
    }
    to {
        transform: rotateY(0deg);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-in".to_string(),
        r#"@keyframes rotate-in {
  from {
    transform: rotate3d(0, 0, 1, -200deg);
    opacity: 0;
  }

  to {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-in;
  transform-origin: center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-left".to_string(),
        r#"@keyframes rotate-left {
    0% {
        transform: rotate(0);
        transform-origin: left
    }
    to {
        transform: rotate(360deg);
        transform-origin: left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-left;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-bck".to_string(),
        r#"@keyframes rotate-out-2-bck {
    0% {
        transform: translateZ(0) rotate(0);
        opacity: 1
    }
    to {
        transform: translateZ(-180px) rotate(-45deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-bl-ccw".to_string(),
        r#"@keyframes rotate-out-2-bl-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 0 100%;
        opacity: 1
    }
    to {
        transform: rotate(-45deg);
        transform-origin: 0 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-bl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-bl-cw".to_string(),
        r#"@keyframes rotate-out-2-bl-cw {
    0% {
        transform: rotate(0);
        transform-origin: 0 100%;
        opacity: 1
    }
    to {
        transform: rotate(45deg);
        transform-origin: 0 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-bl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-br-ccw".to_string(),
        r#"@keyframes rotate-out-2-br-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: rotate(-45deg);
        transform-origin: 100% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-br-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-br-cw".to_string(),
        r#"@keyframes rotate-out-2-br-cw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: rotate(45deg);
        transform-origin: 100% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-br-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-ccw".to_string(),
        r#"@keyframes rotate-out-2-ccw {
    0% {
        transform: rotate(0);
        opacity: 1
    }
    to {
        transform: rotate(-45deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-cw".to_string(),
        r#"@keyframes rotate-out-2-cw {
    0% {
        transform: rotate(0);
        opacity: 1
    }
    to {
        transform: rotate(45deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-fwd".to_string(),
        r#"@keyframes rotate-out-2-fwd {
    0% {
        transform: translateZ(0) rotate(0);
        opacity: 1
    }
    to {
        transform: translateZ(180px) rotate(45deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-tl-ccw".to_string(),
        r#"@keyframes rotate-out-2-tl-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: rotate(-45deg);
        transform-origin: 0 0;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-tl-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-tl-cw".to_string(),
        r#"@keyframes rotate-out-2-tl-cw {
    0% {
        transform: rotate(0);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: rotate(45deg);
        transform-origin: 0 0;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-tl-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-tr-ccw".to_string(),
        r#"@keyframes rotate-out-2-tr-ccw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: rotate(-45deg);
        transform-origin: 100% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-tr-ccw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-2-tr-cw".to_string(),
        r#"@keyframes rotate-out-2-tr-cw {
    0% {
        transform: rotate(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: rotate(45deg);
        transform-origin: 100% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-2-tr-cw;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-bl".to_string(),
        r#"@keyframes rotate-out-bl {
    0% {
        transform: rotate(0);
        transform-origin: bottom left;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: bottom left;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-bottom".to_string(),
        r#"@keyframes rotate-out-bottom {
    0% {
        transform: rotate(0);
        transform-origin: bottom;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: bottom;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-br".to_string(),
        r#"@keyframes rotate-out-br {
    0% {
        transform: rotate(0);
        transform-origin: bottom right;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: bottom right;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-br;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-center".to_string(),
        r#"@keyframes rotate-out-center {
    0% {
        transform: rotate(0);
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-diag-1".to_string(),
        r#"@keyframes rotate-out-diag-1 {
    0% {
        transform: rotate3d(1, 1, 0, 360deg);
        opacity: 1
    }
    to {
        transform: rotate3d(1, 1, 0, 0deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-diag-2".to_string(),
        r#"@keyframes rotate-out-diag-2 {
    0% {
        transform: rotate3d(-1, 1, 0, 360deg);
        opacity: 1
    }
    to {
        transform: rotate3d(-1, 1, 0, 0deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-down-left".to_string(),
        r#"@keyframes rotate-out-down-left {
  from {
    opacity: 1;
  }

  to {
    transform: rotate3d(0, 0, 1, 45deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-down-left;
  transform-origin: left bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-down-right".to_string(),
        r#"@keyframes rotate-out-down-right {
  from {
    opacity: 1;
  }

  to {
    transform: rotate3d(0, 0, 1, -45deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-down-right;
  transform-origin: right bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-hor".to_string(),
        r#"@keyframes rotate-out-hor {
    0% {
        transform: rotateX(360deg);
        opacity: 1
    }
    to {
        transform: rotateX(0deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-left".to_string(),
        r#"@keyframes rotate-out-left {
    0% {
        transform: rotate(0);
        transform-origin: left;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: left;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-right".to_string(),
        r#"@keyframes rotate-out-right {
    0% {
        transform: rotate(0);
        transform-origin: right;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: right;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-tl".to_string(),
        r#"@keyframes rotate-out-tl {
    0% {
        transform: rotate(0);
        transform-origin: top left;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: top left;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-top".to_string(),
        r#"@keyframes rotate-out-top {
    0% {
        transform: rotate(0);
        transform-origin: top;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: top;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-tr".to_string(),
        r#"@keyframes rotate-out-tr {
    0% {
        transform: rotate(0);
        transform-origin: top right;
        opacity: 1
    }
    to {
        transform: rotate(-360deg);
        transform-origin: top right;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-up-left".to_string(),
        r#"@keyframes rotate-out-up-left {
  from {
    opacity: 1;
  }

  to {
    transform: rotate3d(0, 0, 1, -45deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-up-left;
  transform-origin: left bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-up-right".to_string(),
        r#"@keyframes rotate-out-up-right {
  from {
    opacity: 1;
  }

  to {
    transform: rotate3d(0, 0, 1, 90deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-up-right;
  transform-origin: right bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out-ver".to_string(),
        r#"@keyframes rotate-out-ver {
    0% {
        transform: rotateY(360deg);
        opacity: 1
    }
    to {
        transform: rotateY(0deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-out".to_string(),
        r#"@keyframes rotate-out {
  from {
    opacity: 1;
  }

  to {
    transform: rotate3d(0, 0, 1, 200deg);
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-out;
  transform-origin: center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-right".to_string(),
        r#"@keyframes rotate-right {
    0% {
        transform: rotate(0);
        transform-origin: right
    }
    to {
        transform: rotate(360deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-right;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-down-diag-1".to_string(),
        r#"@keyframes rotate-scale-down-diag-1 {
    0% {
        transform: scale(1) rotate3d(1, 1, 0, 0deg)
    }
    50% {
        transform: scale(.5) rotate3d(1, 1, 0, -180deg)
    }
    to {
        transform: scale(1) rotate3d(1, 1, 0, -360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-down-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-down-diag-2".to_string(),
        r#"@keyframes rotate-scale-down-diag-2 {
    0% {
        transform: scale(1) rotate3d(-1, 1, 0, 0deg)
    }
    50% {
        transform: scale(.5) rotate3d(-1, 1, 0, 180deg)
    }
    to {
        transform: scale(1) rotate3d(-1, 1, 0, 360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-down-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-down-hor".to_string(),
        r#"@keyframes rotate-scale-down-hor {
    0% {
        transform: scale(1) rotateX(0)
    }
    50% {
        transform: scale(.5) rotateX(-180deg)
    }
    to {
        transform: scale(1) rotateX(-360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-down-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-down-ver".to_string(),
        r#"@keyframes rotate-scale-down-ver {
    0% {
        transform: scale(1) rotateY(0)
    }
    50% {
        transform: scale(.5) rotateY(180deg)
    }
    to {
        transform: scale(1) rotateY(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-down-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-down".to_string(),
        r#"@keyframes rotate-scale-down {
    0% {
        transform: scale(1) rotateZ(0)
    }
    50% {
        transform: scale(.5) rotateZ(180deg)
    }
    to {
        transform: scale(1) rotateZ(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-down;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-up-diag-1".to_string(),
        r#"@keyframes rotate-scale-up-diag-1 {
    0% {
        transform: scale(1) rotate3d(1, 1, 0, 0deg)
    }
    50% {
        transform: scale(2) rotate3d(1, 1, 0, -180deg)
    }
    to {
        transform: scale(1) rotate3d(1, 1, 0, -360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-up-diag-1;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-up-diag-2".to_string(),
        r#"@keyframes rotate-scale-up-diag-2 {
    0% {
        transform: scale(1) rotate3d(-1, 1, 0, 0deg)
    }
    50% {
        transform: scale(2) rotate3d(-1, 1, 0, 180deg)
    }
    to {
        transform: scale(1) rotate3d(-1, 1, 0, 360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-up-diag-2;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-up-hor".to_string(),
        r#"@keyframes rotate-scale-up-hor {
    0% {
        transform: scale(1) rotateX(0)
    }
    50% {
        transform: scale(2) rotateX(-180deg)
    }
    to {
        transform: scale(1) rotateX(-360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-up-hor;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-up-ver".to_string(),
        r#"@keyframes rotate-scale-up-ver {
    0% {
        transform: scale(1) rotateY(0)
    }
    50% {
        transform: scale(2) rotateY(180deg)
    }
    to {
        transform: scale(1) rotateY(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-up-ver;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-scale-up".to_string(),
        r#"@keyframes rotate-scale-up {
    0% {
        transform: scale(1) rotateZ(0)
    }
    50% {
        transform: scale(2) rotateZ(180deg)
    }
    to {
        transform: scale(1) rotateZ(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-scale-up;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-tl".to_string(),
        r#"@keyframes rotate-tl {
    0% {
        transform: rotate(0);
        transform-origin: top left
    }
    to {
        transform: rotate(360deg);
        transform-origin: top left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-top".to_string(),
        r#"@keyframes rotate-top {
    0% {
        transform: rotate(0);
        transform-origin: top
    }
    to {
        transform: rotate(360deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-top;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-tr".to_string(),
        r#"@keyframes rotate-tr {
    0% {
        transform: rotate(0);
        transform-origin: top right
    }
    to {
        transform: rotate(360deg);
        transform-origin: top right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-vert-center".to_string(),
        r#"@keyframes rotate-vert-center {
    0% {
        transform: rotateY(0)
    }
    to {
        transform: rotateY(360deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-vert-center;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-vert-left".to_string(),
        r#"@keyframes rotate-vert-left {
    0% {
        transform: rotateY(0);
        transform-origin: left
    }
    to {
        transform: rotateY(360deg);
        transform-origin: left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-vert-left;
}
"#
        .to_string(),
    );
    map.insert(
        "rotate-vert-right".to_string(),
        r#"@keyframes rotate-vert-right {
    0% {
        transform: rotateY(0);
        transform-origin: right
    }
    to {
        transform: rotateY(-360deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rotate-vert-right;
}
"#
        .to_string(),
    );
    map.insert(
        "rubber-band".to_string(),
        r#"@keyframes rubber-band {
  from {
    transform: scale3d(1, 1, 1);
  }

  30% {
    transform: scale3d(1.25, 0.75, 1);
  }

  40% {
    transform: scale3d(0.75, 1.25, 1);
  }

  50% {
    transform: scale3d(1.15, 0.85, 1);
  }

  65% {
    transform: scale3d(0.95, 1.05, 1);
  }

  75% {
    transform: scale3d(1.05, 0.95, 1);
  }

  to {
    transform: scale3d(1, 1, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: rubber-band;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-bl".to_string(),
        r#"@keyframes scale-down-bl {
    0% {
        transform: scale(1);
        transform-origin: 0% 100%
    }
    to {
        transform: scale(.5);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-bottom".to_string(),
        r#"@keyframes scale-down-bottom {
    0% {
        transform: scale(1);
        transform-origin: 50% 100%
    }
    to {
        transform: scale(.5);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-br".to_string(),
        r#"@keyframes scale-down-br {
    0% {
        transform: scale(1);
        transform-origin: 100% 100%
    }
    to {
        transform: scale(.5);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-br;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-center".to_string(),
        r#"@keyframes scale-down-center {
    0% {
        transform: scale(1)
    }
    to {
        transform: scale(.5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-hor-center".to_string(),
        r#"@keyframes scale-down-hor-center {
    0% {
        transform: scaleX(1)
    }
    to {
        transform: scaleX(.3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-hor-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-hor-left".to_string(),
        r#"@keyframes scale-down-hor-left {
    0% {
        transform: scaleX(1);
        transform-origin: 0% 0%
    }
    to {
        transform: scaleX(.3);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-hor-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-hor-right".to_string(),
        r#"@keyframes scale-down-hor-right {
    0% {
        transform: scaleX(1);
        transform-origin: 100% 100%
    }
    to {
        transform: scaleX(.3);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-hor-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-left".to_string(),
        r#"@keyframes scale-down-left {
    0% {
        transform: scale(1);
        transform-origin: 0% 50%
    }
    to {
        transform: scale(.5);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-right".to_string(),
        r#"@keyframes scale-down-right {
    0% {
        transform: scale(1);
        transform-origin: 100% 50%
    }
    to {
        transform: scale(.5);
        transform-origin: 100% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-tl".to_string(),
        r#"@keyframes scale-down-tl {
    0% {
        transform: scale(1);
        transform-origin: 0% 0%
    }
    to {
        transform: scale(.5);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-top".to_string(),
        r#"@keyframes scale-down-top {
    0% {
        transform: scale(1);
        transform-origin: 50% 0%
    }
    to {
        transform: scale(.5);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-tr".to_string(),
        r#"@keyframes scale-down-tr {
    0% {
        transform: scale(1);
        transform-origin: 100% 0%
    }
    to {
        transform: scale(.5);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-ver-bottom".to_string(),
        r#"@keyframes scale-down-ver-bottom {
    0% {
        transform: scaleY(1);
        transform-origin: 0% 100%
    }
    to {
        transform: scaleY(.3);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-ver-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-ver-center".to_string(),
        r#"@keyframes scale-down-ver-center {
    0% {
        transform: scaleY(1)
    }
    to {
        transform: scaleY(.3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-ver-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-down-ver-top".to_string(),
        r#"@keyframes scale-down-ver-top {
    0% {
        transform: scaleY(1);
        transform-origin: 100% 0%
    }
    to {
        transform: scaleY(.3);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-down-ver-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-bl".to_string(),
        r#"@keyframes scale-in-bl {
    0% {
        transform: scale(0);
        transform-origin: 0% 100%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 0% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-bottom".to_string(),
        r#"@keyframes scale-in-bottom {
    0% {
        transform: scale(0);
        transform-origin: 50% 100%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-br".to_string(),
        r#"@keyframes scale-in-br {
    0% {
        transform: scale(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-br;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-center".to_string(),
        r#"@keyframes scale-in-center {
    0% {
        transform: scale(0);
        opacity: 1
    }
    to {
        transform: scale(1);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-hor-center".to_string(),
        r#"@keyframes scale-in-hor-center {
    0% {
        transform: scaleX(0);
        opacity: 1
    }
    to {
        transform: scaleX(1);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-hor-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-hor-left".to_string(),
        r#"@keyframes scale-in-hor-left {
    0% {
        transform: scaleX(0);
        transform-origin: 0% 0%;
        opacity: 1
    }
    to {
        transform: scaleX(1);
        transform-origin: 0% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-hor-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-hor-right".to_string(),
        r#"@keyframes scale-in-hor-right {
    0% {
        transform: scaleX(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: scaleX(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-hor-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-left".to_string(),
        r#"@keyframes scale-in-left {
    0% {
        transform: scale(0);
        transform-origin: 0% 50%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 0% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-right".to_string(),
        r#"@keyframes scale-in-right {
    0% {
        transform: scale(0);
        transform-origin: 100% 50%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-tl".to_string(),
        r#"@keyframes scale-in-tl {
    0% {
        transform: scale(0);
        transform-origin: 0% 0%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 0% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-top".to_string(),
        r#"@keyframes scale-in-top {
    0% {
        transform: scale(0);
        transform-origin: 50% 0%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 50% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-tr".to_string(),
        r#"@keyframes scale-in-tr {
    0% {
        transform: scale(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-ver-bottom".to_string(),
        r#"@keyframes scale-in-ver-bottom {
    0% {
        transform: scaleY(0);
        transform-origin: 0% 100%;
        opacity: 1
    }
    to {
        transform: scaleY(1);
        transform-origin: 0% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-ver-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-ver-center".to_string(),
        r#"@keyframes scale-in-ver-center {
    0% {
        transform: scaleY(0);
        opacity: 1
    }
    to {
        transform: scaleY(1);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-ver-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-in-ver-top".to_string(),
        r#"@keyframes scale-in-ver-top {
    0% {
        transform: scaleY(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: scaleY(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-in-ver-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-bl".to_string(),
        r#"@keyframes scale-out-bl {
    0% {
        transform: scale(1);
        transform-origin: 0% 100%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 0% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-bottom".to_string(),
        r#"@keyframes scale-out-bottom {
    0% {
        transform: scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-br".to_string(),
        r#"@keyframes scale-out-br {
    0% {
        transform: scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-br;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-center".to_string(),
        r#"@keyframes scale-out-center {
    0% {
        transform: scale(1);
        opacity: 1
    }
    to {
        transform: scale(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-hor-left".to_string(),
        r#"@keyframes scale-out-hor-left {
    0% {
        transform: scaleX(1);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: scaleX(0);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-hor-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-hor-right".to_string(),
        r#"@keyframes scale-out-hor-right {
    0% {
        transform: scaleX(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: scaleX(0);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-hor-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-horizontal".to_string(),
        r#"@keyframes scale-out-horizontal {
    0% {
        transform: scaleX(1);
        opacity: 1
    }
    to {
        transform: scaleX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-horizontal;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-left".to_string(),
        r#"@keyframes scale-out-left {
    0% {
        transform: scale(1);
        transform-origin: 0% 50%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 0% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-right".to_string(),
        r#"@keyframes scale-out-right {
    0% {
        transform: scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-tl".to_string(),
        r#"@keyframes scale-out-tl {
    0% {
        transform: scale(1);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-top".to_string(),
        r#"@keyframes scale-out-top {
    0% {
        transform: scale(1);
        transform-origin: 50% 0%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 50% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-tr".to_string(),
        r#"@keyframes scale-out-tr {
    0% {
        transform: scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: scale(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-ver-bottom".to_string(),
        r#"@keyframes scale-out-ver-bottom {
    0% {
        transform: scaleY(1);
        transform-origin: 0% 100%;
        opacity: 1
    }
    to {
        transform: scaleY(0);
        transform-origin: 0% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-ver-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-ver-top".to_string(),
        r#"@keyframes scale-out-ver-top {
    0% {
        transform: scaleY(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: scaleY(0);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-ver-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-out-vertical".to_string(),
        r#"@keyframes scale-out-vertical {
    0% {
        transform: scaleY(1);
        opacity: 1
    }
    to {
        transform: scaleY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-out-vertical;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-bl".to_string(),
        r#"@keyframes scale-up-bl {
    0% {
        transform: scale(.5);
        transform-origin: 0% 100%
    }
    to {
        transform: scale(1);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-bottom".to_string(),
        r#"@keyframes scale-up-bottom {
    0% {
        transform: scale(.5);
        transform-origin: 50% 100%
    }
    to {
        transform: scale(1);
        transform-origin: 50% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-br".to_string(),
        r#"@keyframes scale-up-br {
    0% {
        transform: scale(.5);
        transform-origin: 100% 100%
    }
    to {
        transform: scale(1);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-br;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-center".to_string(),
        r#"@keyframes scale-up-center {
    0% {
        transform: scale(.5)
    }
    to {
        transform: scale(1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-hor-center".to_string(),
        r#"@keyframes scale-up-hor-center {
    0% {
        transform: scaleX(.4)
    }
    to {
        transform: scaleX(1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-hor-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-hor-left".to_string(),
        r#"@keyframes scale-up-hor-left {
    0% {
        transform: scaleX(.4);
        transform-origin: 0% 0%
    }
    to {
        transform: scaleX(1);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-hor-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-hor-right".to_string(),
        r#"@keyframes scale-up-hor-right {
    0% {
        transform: scaleX(.4);
        transform-origin: 100% 100%
    }
    to {
        transform: scaleX(1);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-hor-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-left".to_string(),
        r#"@keyframes scale-up-left {
    0% {
        transform: scale(.5);
        transform-origin: 0% 50%
    }
    to {
        transform: scale(1);
        transform-origin: 0% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-left;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-right".to_string(),
        r#"@keyframes scale-up-right {
    0% {
        transform: scale(.5);
        transform-origin: 100% 50%
    }
    to {
        transform: scale(1);
        transform-origin: 100% 50%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-right;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-tl".to_string(),
        r#"@keyframes scale-up-tl {
    0% {
        transform: scale(.5);
        transform-origin: 0% 0%
    }
    to {
        transform: scale(1);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-top".to_string(),
        r#"@keyframes scale-up-top {
    0% {
        transform: scale(.5);
        transform-origin: 50% 0%
    }
    to {
        transform: scale(1);
        transform-origin: 50% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-top;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-tr".to_string(),
        r#"@keyframes scale-up-tr {
    0% {
        transform: scale(.5);
        transform-origin: 100% 0%
    }
    to {
        transform: scale(1);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-ver-bottom".to_string(),
        r#"@keyframes scale-up-ver-bottom {
    0% {
        transform: scaleY(.4);
        transform-origin: 0% 100%
    }
    to {
        transform: scaleY(1);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-ver-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-ver-center".to_string(),
        r#"@keyframes scale-up-ver-center {
    0% {
        transform: scaleY(.4)
    }
    to {
        transform: scaleY(1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-ver-center;
}
"#
        .to_string(),
    );
    map.insert(
        "scale-up-ver-top".to_string(),
        r#"@keyframes scale-up-ver-top {
    0% {
        transform: scaleY(.4);
        transform-origin: 100% 0%
    }
    to {
        transform: scaleY(1);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: scale-up-ver-top;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-bl".to_string(),
        r#"@keyframes shadow-drop-2-bl {
    0% {
        transform: translateZ(0) translateX(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(12px) translateY(-12px);
        box-shadow: -12px 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-bottom".to_string(),
        r#"@keyframes shadow-drop-2-bottom {
    0% {
        transform: translateZ(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateY(-12px);
        box-shadow: 0 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-br".to_string(),
        r#"@keyframes shadow-drop-2-br {
    0% {
        transform: translateZ(0) translateX(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(-12px) translateY(-12px);
        box-shadow: 12px 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-br;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-center".to_string(),
        r#"@keyframes shadow-drop-2-center {
    0% {
        transform: translateZ(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px);
        box-shadow: 0 0 20px 0 rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-center;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-left".to_string(),
        r#"@keyframes shadow-drop-2-left {
    0% {
        transform: translateZ(0) translateX(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(12px);
        box-shadow: -12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-left;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-lr".to_string(),
        r#"@keyframes shadow-drop-2-lr {
    0% {
        transform: translateZ(0);
        box-shadow: 0 0 0 0 transparent, 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px);
        box-shadow: -12px 0 20px -12px rgba(0, 0, 0, .35), 12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-lr;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-right".to_string(),
        r#"@keyframes shadow-drop-2-right {
    0% {
        transform: translateZ(0) translateX(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(-12px);
        box-shadow: 12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-right;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-tb".to_string(),
        r#"@keyframes shadow-drop-2-tb {
    0% {
        transform: translateZ(0);
        box-shadow: 0 0 0 0 transparent, 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px);
        box-shadow: 0 -12px 20px -12px rgba(0, 0, 0, .35), 0 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-tb;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-tl".to_string(),
        r#"@keyframes shadow-drop-2-tl {
    0% {
        transform: translateZ(0) translateX(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(12px) translateY(12px);
        box-shadow: -12px -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-top".to_string(),
        r#"@keyframes shadow-drop-2-top {
    0% {
        transform: translateZ(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateY(12px);
        box-shadow: 0 -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-top;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-2-tr".to_string(),
        r#"@keyframes shadow-drop-2-tr {
    0% {
        transform: translateZ(0) translateX(0) translateY(0);
        box-shadow: 0 0 0 0 transparent
    }
    to {
        transform: translateZ(50px) translateX(-12px) translateY(12px);
        box-shadow: 12px -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-2-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-bl".to_string(),
        r#"@keyframes shadow-drop-bl {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: -12px 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-bottom".to_string(),
        r#"@keyframes shadow-drop-bottom {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 0 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-br".to_string(),
        r#"@keyframes shadow-drop-br {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 12px 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-br;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-center".to_string(),
        r#"@keyframes shadow-drop-center {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 0 0 20px 0 rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-center;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-left".to_string(),
        r#"@keyframes shadow-drop-left {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: -12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-left;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-lr".to_string(),
        r#"@keyframes shadow-drop-lr {
    0% {
        box-shadow: 0 0 0 0 transparent, 0 0 0 0 transparent
    }
    to {
        box-shadow: -12px 0 20px -12px rgba(0, 0, 0, .35), 12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-lr;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-right".to_string(),
        r#"@keyframes shadow-drop-right {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 12px 0 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-right;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-tb".to_string(),
        r#"@keyframes shadow-drop-tb {
    0% {
        box-shadow: 0 0 0 0 transparent, 0 0 0 0 transparent
    }
    to {
        box-shadow: 0 -12px 20px -12px rgba(0, 0, 0, .35), 0 12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-tb;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-tl".to_string(),
        r#"@keyframes shadow-drop-tl {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: -12px -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-top".to_string(),
        r#"@keyframes shadow-drop-top {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 0 -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-top;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-drop-tr".to_string(),
        r#"@keyframes shadow-drop-tr {
    0% {
        box-shadow: 0 0 0 0 transparent
    }
    to {
        box-shadow: 12px -12px 20px -12px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-drop-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-bl".to_string(),
        r#"@keyframes shadow-inset-bl {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 6px -6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-bottom".to_string(),
        r#"@keyframes shadow-inset-bottom {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 0 -6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-br".to_string(),
        r#"@keyframes shadow-inset-br {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset -6px -6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-br;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-center".to_string(),
        r#"@keyframes shadow-inset-center {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 0 0 14px 0 rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-center;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-left".to_string(),
        r#"@keyframes shadow-inset-left {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 6px 0 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-left;
}
"#
        .to_string(),
    );
    map.insert("shadow-inset-lr".to_string(), r#"@keyframes shadow-inset-lr {
    0% {
        box-shadow: inset 0 0 0 0 transparent, inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset -6px 0 14px -6px rgba(0, 0, 0, .5), inset 6px 0 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-lr;
}
"#.to_string());
    map.insert(
        "shadow-inset-right".to_string(),
        r#"@keyframes shadow-inset-right {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset -6px 0 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-right;
}
"#
        .to_string(),
    );
    map.insert("shadow-inset-tb".to_string(), r#"@keyframes shadow-inset-tb {
    0% {
        box-shadow: inset 0 0 0 0 transparent, inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 0 -6px 14px -6px rgba(0, 0, 0, .5), inset 0 6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-tb;
}
"#.to_string());
    map.insert(
        "shadow-inset-tl".to_string(),
        r#"@keyframes shadow-inset-tl {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 6px 6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-top".to_string(),
        r#"@keyframes shadow-inset-top {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset 0 6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-top;
}
"#
        .to_string(),
    );
    map.insert(
        "shadow-inset-tr".to_string(),
        r#"@keyframes shadow-inset-tr {
    0% {
        box-shadow: inset 0 0 0 0 transparent
    }
    to {
        box-shadow: inset -6px 6px 14px -6px rgba(0, 0, 0, .5)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-inset-tr;
}
"#
        .to_string(),
    );
    map.insert("shadow-pop-bl".to_string(), r#"@keyframes shadow-pop-bl {
    0% {
        box-shadow: 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e;
        transform: translateX(0) translateY(0)
    }
    to {
        box-shadow: -1px 1px #3e3e3e, -2px 2px #3e3e3e, -3px 3px #3e3e3e, -4px 4px #3e3e3e, -5px 5px #3e3e3e, -6px 6px #3e3e3e, -7px 7px #3e3e3e, -8px 8px #3e3e3e;
        transform: translateX(8px) translateY(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-pop-bl;
}
"#.to_string());
    map.insert("shadow-pop-br".to_string(), r#"@keyframes shadow-pop-br {
    0% {
        box-shadow: 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e;
        transform: translateX(0) translateY(0)
    }
    to {
        box-shadow: 1px 1px #3e3e3e, 2px 2px #3e3e3e, 3px 3px #3e3e3e, 4px 4px #3e3e3e, 5px 5px #3e3e3e, 6px 6px #3e3e3e, 7px 7px #3e3e3e, 8px 8px #3e3e3e;
        transform: translateX(-8px) translateY(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-pop-br;
}
"#.to_string());
    map.insert("shadow-pop-tl".to_string(), r#"@keyframes shadow-pop-tl {
    0% {
        box-shadow: 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e;
        transform: translateX(0) translateY(0)
    }
    to {
        box-shadow: -1px -1px #3e3e3e, -2px -2px #3e3e3e, -3px -3px #3e3e3e, -4px -4px #3e3e3e, -5px -5px #3e3e3e, -6px -6px #3e3e3e, -7px -7px #3e3e3e, -8px -8px #3e3e3e;
        transform: translateX(8px) translateY(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-pop-tl;
}
"#.to_string());
    map.insert("shadow-pop-tr".to_string(), r#"@keyframes shadow-pop-tr {
    0% {
        box-shadow: 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e, 0 0 #3e3e3e;
        transform: translateX(0) translateY(0)
    }
    to {
        box-shadow: 1px -1px #3e3e3e, 2px -2px #3e3e3e, 3px -3px #3e3e3e, 4px -4px #3e3e3e, 5px -5px #3e3e3e, 6px -6px #3e3e3e, 7px -7px #3e3e3e, 8px -8px #3e3e3e;
        transform: translateX(-8px) translateY(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shadow-pop-tr;
}
"#.to_string());
    map.insert(
        "shake-bl".to_string(),
        r#"@keyframes shake-bl {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 0 100%
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-bottom".to_string(),
        r#"@keyframes shake-bottom {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 50% 100%
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-br".to_string(),
        r#"@keyframes shake-br {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 100% 100%
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-br;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-horizontal".to_string(),
        r#"@keyframes shake-horizontal {
    0%,
    to {
        transform: translateX(0)
    }
    10%,
    30%,
    50%,
    70% {
        transform: translateX(-10px)
    }
    20%,
    40%,
    60% {
        transform: translateX(10px)
    }
    80% {
        transform: translateX(8px)
    }
    90% {
        transform: translateX(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-horizontal;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-left".to_string(),
        r#"@keyframes shake-left {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 0 50%
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-left;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-lr".to_string(),
        r#"@keyframes shake-lr {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 50% 50%
    }
    10%,
    90% {
        transform: rotate(8deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-10deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(10deg)
    }
    80% {
        transform: rotate(-8deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-lr;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-right".to_string(),
        r#"@keyframes shake-right {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 100% 50%
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-right;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-tl".to_string(),
        r#"@keyframes shake-tl {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 0 0
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-top".to_string(),
        r#"@keyframes shake-top {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 50% 0
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-top;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-tr".to_string(),
        r#"@keyframes shake-tr {
    0%,
    to {
        transform: rotate(0deg);
        transform-origin: 100% 0
    }
    10%,
    90% {
        transform: rotate(2deg)
    }
    20%,
    40%,
    60% {
        transform: rotate(-4deg)
    }
    30%,
    50%,
    70% {
        transform: rotate(4deg)
    }
    80% {
        transform: rotate(-2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-vertical".to_string(),
        r#"@keyframes shake-vertical {
    0%,
    to {
        transform: translateY(0)
    }
    10%,
    30%,
    50%,
    70% {
        transform: translateY(-8px)
    }
    20%,
    40%,
    60% {
        transform: translateY(8px)
    }
    80% {
        transform: translateY(6.4px)
    }
    90% {
        transform: translateY(-6.4px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-vertical;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-x".to_string(),
        r#"@keyframes shake-x {
  from,
  to {
    transform: translate3d(0, 0, 0);
  }

  10%,
  30%,
  50%,
  70%,
  90% {
    transform: translate3d(-10px, 0, 0);
  }

  20%,
  40%,
  60%,
  80% {
    transform: translate3d(10px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-x;
}
"#
        .to_string(),
    );
    map.insert(
        "shake-y".to_string(),
        r#"@keyframes shake-y {
  from,
  to {
    transform: translate3d(0, 0, 0);
  }

  10%,
  30%,
  50%,
  70%,
  90% {
    transform: translate3d(0, -10px, 0);
  }

  20%,
  40%,
  60%,
  80% {
    transform: translate3d(0, 10px, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake-y;
}
"#
        .to_string(),
    );
    map.insert(
        "shake".to_string(),
        r#"@keyframes shake {
  from,
  to {
    transform: translate3d(0, 0, 0);
  }

  10%,
  30%,
  50%,
  70%,
  90% {
    transform: translate3d(-10px, 0, 0);
  }

  20%,
  40%,
  60%,
  80% {
    transform: translate3d(10px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: shake;
}
"#
        .to_string(),
    );
    map.insert(
        "simple-fade-in".to_string(),
        r#"@keyframes simple-fade-in {
    0% {
        opacity: 0
    }
    to {
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: simple-fade-in;
}
"#
        .to_string(),
    );
    map.insert(
        "simple-fade-out".to_string(),
        r#"@keyframes simple-fade-out {
    0% {
        opacity: 1
    }
    to {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: simple-fade-out;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-bl".to_string(),
        r#"@keyframes slide-bck-bl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateY(200px) translateX(-200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-bottom".to_string(),
        r#"@keyframes slide-bck-bottom {
    0% {
        transform: translateZ(0) translateY(0)
    }
    to {
        transform: translateZ(-400px) translateY(200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-br".to_string(),
        r#"@keyframes slide-bck-br {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateY(200px) translateX(200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-center".to_string(),
        r#"@keyframes slide-bck-center {
    0% {
        transform: translateZ(0)
    }
    to {
        transform: translateZ(-400px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-left".to_string(),
        r#"@keyframes slide-bck-left {
    0% {
        transform: translateZ(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateX(-200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-right".to_string(),
        r#"@keyframes slide-bck-right {
    0% {
        transform: translateZ(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateX(200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-tl".to_string(),
        r#"@keyframes slide-bck-tl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateY(-200px) translateX(-200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-top".to_string(),
        r#"@keyframes slide-bck-top {
    0% {
        transform: translateZ(0) translateY(0)
    }
    to {
        transform: translateZ(-400px) translateY(-200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bck-tr".to_string(),
        r#"@keyframes slide-bck-tr {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(-400px) translateY(-200px) translateX(200px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bck-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bl".to_string(),
        r#"@keyframes slide-bl {
    0% {
        transform: translateY(0) translateX(0)
    }
    to {
        transform: translateY(100px) translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-bottom".to_string(),
        r#"@keyframes slide-bottom {
    0% {
        transform: translateY(0)
    }
    to {
        transform: translateY(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-br".to_string(),
        r#"@keyframes slide-br {
    0% {
        transform: translateY(0) translateX(0)
    }
    to {
        transform: translateY(100px) translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-bl".to_string(),
        r#"@keyframes slide-fwd-bl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateY(100px) translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-bottom".to_string(),
        r#"@keyframes slide-fwd-bottom {
    0% {
        transform: translateZ(0) translateY(0)
    }
    to {
        transform: translateZ(160px) translateY(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-br".to_string(),
        r#"@keyframes slide-fwd-br {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateY(100px) translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-center".to_string(),
        r#"@keyframes slide-fwd-center {
    0% {
        transform: translateZ(0)
    }
    to {
        transform: translateZ(160px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-left".to_string(),
        r#"@keyframes slide-fwd-left {
    0% {
        transform: translateZ(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-right".to_string(),
        r#"@keyframes slide-fwd-right {
    0% {
        transform: translateZ(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-tl".to_string(),
        r#"@keyframes slide-fwd-tl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateY(-100px) translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-top".to_string(),
        r#"@keyframes slide-fwd-top {
    0% {
        transform: translateZ(0) translateY(0)
    }
    to {
        transform: translateZ(160px) translateY(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-fwd-tr".to_string(),
        r#"@keyframes slide-fwd-tr {
    0% {
        transform: translateZ(0) translateY(0) translateX(0)
    }
    to {
        transform: translateZ(160px) translateY(-100px) translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-fwd-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-bl".to_string(),
        r#"@keyframes slide-in-bck-bl {
    0% {
        transform: translateZ(700px) translateY(300px) translateX(-400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-bottom".to_string(),
        r#"@keyframes slide-in-bck-bottom {
    0% {
        transform: translateZ(700px) translateY(300px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-br".to_string(),
        r#"@keyframes slide-in-bck-br {
    0% {
        transform: translateZ(700px) translateY(300px) translateX(400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-center".to_string(),
        r#"@keyframes slide-in-bck-center {
    0% {
        transform: translateZ(600px);
        opacity: 0
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-left".to_string(),
        r#"@keyframes slide-in-bck-left {
    0% {
        transform: translateZ(700px) translateX(-400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-right".to_string(),
        r#"@keyframes slide-in-bck-right {
    0% {
        transform: translateZ(700px) translateX(400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-tl".to_string(),
        r#"@keyframes slide-in-bck-tl {
    0% {
        transform: translateZ(700px) translateY(-300px) translateX(-400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-top".to_string(),
        r#"@keyframes slide-in-bck-top {
    0% {
        transform: translateZ(700px) translateY(-300px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bck-tr".to_string(),
        r#"@keyframes slide-in-bck-tr {
    0% {
        transform: translateZ(700px) translateY(-300px) translateX(400px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bck-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bl".to_string(),
        r#"@keyframes slide-in-bl {
    0% {
        transform: translateY(1000px) translateX(-1000px);
        opacity: 0
    }
    to {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-bl".to_string(),
        r#"@keyframes slide-in-blurred-bl {
    0% {
        transform: translate(-1000px, 1000px) skew(-80deg, -10deg);
        transform-origin: 100% 100%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-bottom".to_string(),
        r#"@keyframes slide-in-blurred-bottom {
    0% {
        transform: translateY(1000px) scaleY(2.5) scaleX(.2);
        transform-origin: 50% 100%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translateY(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-br".to_string(),
        r#"@keyframes slide-in-blurred-br {
    0% {
        transform: translate(1000px, 1000px) skew(80deg, 10deg);
        transform-origin: 0% 100%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-left".to_string(),
        r#"@keyframes slide-in-blurred-left {
    0% {
        transform: translateX(-1000px) scaleX(2.5) scaleY(.2);
        transform-origin: 100% 50%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translateX(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-right".to_string(),
        r#"@keyframes slide-in-blurred-right {
    0% {
        transform: translateX(1000px) scaleX(2.5) scaleY(.2);
        transform-origin: 0% 50%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translateX(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-tl".to_string(),
        r#"@keyframes slide-in-blurred-tl {
    0% {
        transform: translate(-1000px, -1000px) skew(80deg, 10deg);
        transform-origin: 100% 0%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-top".to_string(),
        r#"@keyframes slide-in-blurred-top {
    0% {
        transform: translateY(-1000px) scaleY(2.5) scaleX(.2);
        transform-origin: 50% 0%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translateY(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-blurred-tr".to_string(),
        r#"@keyframes slide-in-blurred-tr {
    0% {
        transform: translate(1000px, -1000px) skew(-80deg, -10deg);
        transform-origin: 0% 0%;
        filter: blur(40px);
        opacity: 0
    }
    to {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-blurred-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-bottom".to_string(),
        r#"@keyframes slide-in-bottom {
    0% {
        transform: translateY(1000px);
        opacity: 0
    }
    to {
        transform: translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-br".to_string(),
        r#"@keyframes slide-in-br {
    0% {
        transform: translateY(1000px) translateX(1000px);
        opacity: 0
    }
    to {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-down".to_string(),
        r#"@keyframes slide-in-down {
  from {
    transform: translate3d(0, -100%, 0);
    visibility: visible;
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-down;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-bottom-bck".to_string(),
        r#"@keyframes slide-in-elliptic-bottom-bck {
    0% {
        transform: translateY(600px) rotateX(-30deg) scale(6.5);
        transform-origin: 50% -100%;
        opacity: 0
    }
    to {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 500px;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-bottom-fwd".to_string(),
        r#"@keyframes slide-in-elliptic-bottom-fwd {
    0% {
        transform: translateY(600px) rotateX(30deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 0
    }
    to {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% -1400px;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-left-bck".to_string(),
        r#"@keyframes slide-in-elliptic-left-bck {
    0% {
        transform: translateX(-800px) rotateY(-30deg) scale(6.5);
        transform-origin: 200% 50%;
        opacity: 0
    }
    to {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: -600px 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-left-fwd".to_string(),
        r#"@keyframes slide-in-elliptic-left-fwd {
    0% {
        transform: translateX(-800px) rotateY(30deg) scale(0);
        transform-origin: -100% 50%;
        opacity: 0
    }
    to {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 1800px 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-right-bck".to_string(),
        r#"@keyframes slide-in-elliptic-right-bck {
    0% {
        transform: translateX(800px) rotateY(30deg) scale(6.5);
        transform-origin: -100% 50%;
        opacity: 0
    }
    to {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 600px 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-right-fwd".to_string(),
        r#"@keyframes slide-in-elliptic-right-fwd {
    0% {
        transform: translateX(800px) rotateY(-30deg) scale(0);
        transform-origin: -100% 50%;
        opacity: 0
    }
    to {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: -1800px 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-top-bck".to_string(),
        r#"@keyframes slide-in-elliptic-top-bck {
    0% {
        transform: translateY(-600px) rotateX(30deg) scale(6.5);
        transform-origin: 50% 200%;
        opacity: 0
    }
    to {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% -500px;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-elliptic-top-fwd".to_string(),
        r#"@keyframes slide-in-elliptic-top-fwd {
    0% {
        transform: translateY(-600px) rotateX(-30deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 0
    }
    to {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 1400px;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-elliptic-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-bl".to_string(),
        r#"@keyframes slide-in-fwd-bl {
    0% {
        transform: translateZ(-1400px) translateY(800px) translateX(-1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-bottom".to_string(),
        r#"@keyframes slide-in-fwd-bottom {
    0% {
        transform: translateZ(-1400px) translateY(800px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-br".to_string(),
        r#"@keyframes slide-in-fwd-br {
    0% {
        transform: translateZ(-1400px) translateY(800px) translateX(1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-center".to_string(),
        r#"@keyframes slide-in-fwd-center {
    0% {
        transform: translateZ(-1400px);
        opacity: 0
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-left".to_string(),
        r#"@keyframes slide-in-fwd-left {
    0% {
        transform: translateZ(-1400px) translateX(-1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-right".to_string(),
        r#"@keyframes slide-in-fwd-right {
    0% {
        transform: translateZ(-1400px) translateX(1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-tl".to_string(),
        r#"@keyframes slide-in-fwd-tl {
    0% {
        transform: translateZ(-1400px) translateY(-800px) translateX(-1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-top".to_string(),
        r#"@keyframes slide-in-fwd-top {
    0% {
        transform: translateZ(-1400px) translateY(-800px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-fwd-tr".to_string(),
        r#"@keyframes slide-in-fwd-tr {
    0% {
        transform: translateZ(-1400px) translateY(-800px) translateX(1000px);
        opacity: 0
    }
    to {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-fwd-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-left".to_string(),
        r#"@keyframes slide-in-left {
  from {
    transform: translate3d(-100%, 0, 0);
    visibility: visible;
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-right".to_string(),
        r#"@keyframes slide-in-right {
  from {
    transform: translate3d(100%, 0, 0);
    visibility: visible;
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-tl".to_string(),
        r#"@keyframes slide-in-tl {
    0% {
        transform: translateY(-1000px) translateX(-1000px);
        opacity: 0
    }
    to {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-top".to_string(),
        r#"@keyframes slide-in-top {
    0% {
        transform: translateY(-1000px);
        opacity: 0
    }
    to {
        transform: translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-tr".to_string(),
        r#"@keyframes slide-in-tr {
    0% {
        transform: translateY(-1000px) translateX(1000px);
        opacity: 0
    }
    to {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-in-up".to_string(),
        r#"@keyframes slide-in-up {
  from {
    transform: translate3d(0, 100%, 0);
    visibility: visible;
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-in-up;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-left".to_string(),
        r#"@keyframes slide-left {
    0% {
        transform: translateX(0)
    }
    to {
        transform: translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-bl".to_string(),
        r#"@keyframes slide-out-bck-bl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(1000px) translateX(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-bottom".to_string(),
        r#"@keyframes slide-out-bck-bottom {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-br".to_string(),
        r#"@keyframes slide-out-bck-br {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(1000px) translateX(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-center".to_string(),
        r#"@keyframes slide-out-bck-center {
    0% {
        transform: translateZ(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-left".to_string(),
        r#"@keyframes slide-out-bck-left {
    0% {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateX(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-right".to_string(),
        r#"@keyframes slide-out-bck-right {
    0% {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateX(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-tl".to_string(),
        r#"@keyframes slide-out-bck-tl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(-1000px) translateX(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-top".to_string(),
        r#"@keyframes slide-out-bck-top {
    0% {
        transform: translateZ(1) translateY(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bck-tr".to_string(),
        r#"@keyframes slide-out-bck-tr {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(-1100px) translateY(-1000px) translateX(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bck-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bl".to_string(),
        r#"@keyframes slide-out-bl {
    0% {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateY(1000px) translateX(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-bl".to_string(),
        r#"@keyframes slide-out-blurred-bl {
    0% {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translate(-1000px, 1000px) skew(-80deg, -10deg);
        transform-origin: 100% 100%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-bottom".to_string(),
        r#"@keyframes slide-out-blurred-bottom {
    0% {
        transform: translateY(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateY(1000px) scaleY(2) scaleX(.2);
        transform-origin: 50% 100%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-br".to_string(),
        r#"@keyframes slide-out-blurred-br {
    0% {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translate(1000px, 1000px) skew(80deg, 10deg);
        transform-origin: 0% 100%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-left".to_string(),
        r#"@keyframes slide-out-blurred-left {
    0% {
        transform: translateX(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateX(-1000px) scaleX(2) scaleY(.2);
        transform-origin: 100% 50%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-right".to_string(),
        r#"@keyframes slide-out-blurred-right {
    0% {
        transform: translateX(0) scaleY(1) scaleX(1);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateX(1000px) scaleX(2) scaleY(.2);
        transform-origin: 0% 50%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-tl".to_string(),
        r#"@keyframes slide-out-blurred-tl {
    0% {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translate(-1000px, -1000px) skew(80deg, 10deg);
        transform-origin: 100% 0%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-top".to_string(),
        r#"@keyframes slide-out-blurred-top {
    0% {
        transform: translateY(0) scaleY(1) scaleX(1);
        transform-origin: 50% 0%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translateY(-1000px) scaleY(2) scaleX(.2);
        transform-origin: 50% 0%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-blurred-tr".to_string(),
        r#"@keyframes slide-out-blurred-tr {
    0% {
        transform: translate(0, 0) skew(0deg, 0deg);
        transform-origin: 50% 50%;
        filter: blur(0);
        opacity: 1
    }
    to {
        transform: translate(1000px, -1000px) skew(-80deg, -10deg);
        transform-origin: 0% 0%;
        filter: blur(40px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-blurred-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-bottom".to_string(),
        r#"@keyframes slide-out-bottom {
    0% {
        transform: translateY(0);
        opacity: 1
    }
    to {
        transform: translateY(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-br".to_string(),
        r#"@keyframes slide-out-br {
    0% {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateY(1000px) translateX(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-down".to_string(),
        r#"@keyframes slide-out-down {
  from {
    transform: translate3d(0, 0, 0);
  }

  to {
    visibility: hidden;
    transform: translate3d(0, 100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-down;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-bottom-bck".to_string(),
        r#"@keyframes slide-out-elliptic-bottom-bck {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% -1400px;
        opacity: 1
    }
    to {
        transform: translateY(600px) rotateX(30deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-bottom-fwd".to_string(),
        r#"@keyframes slide-out-elliptic-bottom-fwd {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 500px;
        opacity: 1
    }
    to {
        transform: translateY(600px) rotateX(-20deg) scale(6);
        transform-origin: 50% -100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-left-bck".to_string(),
        r#"@keyframes slide-out-elliptic-left-bck {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 2000px 50%;
        opacity: 1
    }
    to {
        transform: translateX(-1000px) rotateY(30deg) scale(0);
        transform-origin: -100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-left-fwd".to_string(),
        r#"@keyframes slide-out-elliptic-left-fwd {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: -500px 50%;
        opacity: 1
    }
    to {
        transform: translateX(-1000px) rotateY(-20deg) scale(6);
        transform-origin: 200% 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-right-bck".to_string(),
        r#"@keyframes slide-out-elliptic-right-bck {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: -1800px 50%;
        opacity: 1
    }
    to {
        transform: translateX(1000px) rotateY(-30deg) scale(0);
        transform-origin: -100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-right-fwd".to_string(),
        r#"@keyframes slide-out-elliptic-right-fwd {
    0% {
        transform: translateX(0) rotateY(0) scale(1);
        transform-origin: 600px 50%;
        opacity: 1
    }
    to {
        transform: translateX(1000px) rotateY(20deg) scale(6);
        transform-origin: -100% 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-top-bck".to_string(),
        r#"@keyframes slide-out-elliptic-top-bck {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% 1400px;
        opacity: 1
    }
    to {
        transform: translateY(-600px) rotateX(-30deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-elliptic-top-fwd".to_string(),
        r#"@keyframes slide-out-elliptic-top-fwd {
    0% {
        transform: translateY(0) rotateX(0) scale(1);
        transform-origin: 50% -500px;
        opacity: 1
    }
    to {
        transform: translateY(-600px) rotateX(20deg) scale(6);
        transform-origin: 50% 200%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-elliptic-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-bl".to_string(),
        r#"@keyframes slide-out-fwd-bl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(300px) translateX(-400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-bottom".to_string(),
        r#"@keyframes slide-out-fwd-bottom {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(300px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-br".to_string(),
        r#"@keyframes slide-out-fwd-br {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(300px) translateX(400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-br;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-center".to_string(),
        r#"@keyframes slide-out-fwd-center {
    0% {
        transform: translateZ(1);
        opacity: 1
    }
    to {
        transform: translateZ(600px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-center;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-left".to_string(),
        r#"@keyframes slide-out-fwd-left {
    0% {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateX(-400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-right".to_string(),
        r#"@keyframes slide-out-fwd-right {
    0% {
        transform: translateZ(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateX(400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-tl".to_string(),
        r#"@keyframes slide-out-fwd-tl {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(-300px) translateX(-400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-top".to_string(),
        r#"@keyframes slide-out-fwd-top {
    0% {
        transform: translateZ(1) translateY(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(-300px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-fwd-tr".to_string(),
        r#"@keyframes slide-out-fwd-tr {
    0% {
        transform: translateZ(0) translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateZ(600px) translateY(-300px) translateX(400px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-fwd-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-left".to_string(),
        r#"@keyframes slide-out-left {
  from {
    transform: translate3d(0, 0, 0);
  }

  to {
    visibility: hidden;
    transform: translate3d(-100%, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-right".to_string(),
        r#"@keyframes slide-out-right {
  from {
    transform: translate3d(0, 0, 0);
  }

  to {
    visibility: hidden;
    transform: translate3d(100%, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-tl".to_string(),
        r#"@keyframes slide-out-tl {
    0% {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateY(-1000px) translateX(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-top".to_string(),
        r#"@keyframes slide-out-top {
    0% {
        transform: translateY(0);
        opacity: 1
    }
    to {
        transform: translateY(-1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-tr".to_string(),
        r#"@keyframes slide-out-tr {
    0% {
        transform: translateY(0) translateX(0);
        opacity: 1
    }
    to {
        transform: translateY(-1000px) translateX(1000px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-out-up".to_string(),
        r#"@keyframes slide-out-up {
  from {
    transform: translate3d(0, 0, 0);
  }

  to {
    visibility: hidden;
    transform: translate3d(0, -100%, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-out-up;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-right".to_string(),
        r#"@keyframes slide-right {
    0% {
        transform: translateX(0)
    }
    to {
        transform: translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-b-bck".to_string(),
        r#"@keyframes slide-rotate-hor-b-bck {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0deg);
        transform-origin: bottom center
    }
    to {
        transform: translateY(150px) translateZ(-230px) rotateX(90deg);
        transform-origin: bottom center
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-b-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-b-fwd".to_string(),
        r#"@keyframes slide-rotate-hor-b-fwd {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0deg);
        transform-origin: top center
    }
    to {
        transform: translateY(150px) translateZ(130px) rotateX(90deg);
        transform-origin: top center
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-b-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-bottom".to_string(),
        r#"@keyframes slide-rotate-hor-bottom {
    0% {
        transform: translateY(0) rotateX(0deg)
    }
    to {
        transform: translateY(150px) rotateX(90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-t-bck".to_string(),
        r#"@keyframes slide-rotate-hor-t-bck {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0deg);
        transform-origin: top center
    }
    to {
        transform: translateY(-150px) translateZ(-230px) rotateX(-90deg);
        transform-origin: top center
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-t-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-t-fwd".to_string(),
        r#"@keyframes slide-rotate-hor-t-fwd {
    0% {
        transform: translateY(0) translateZ(0) rotateX(0deg);
        transform-origin: bottom center
    }
    to {
        transform: translateY(-150px) translateZ(130px) rotateX(-90deg);
        transform-origin: bottom center
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-t-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-hor-top".to_string(),
        r#"@keyframes slide-rotate-hor-top {
    0% {
        transform: translateY(0) rotateX(0deg)
    }
    to {
        transform: translateY(-150px) rotateX(-90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-l-bck".to_string(),
        r#"@keyframes slide-rotate-ver-l-bck {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: center left
    }
    to {
        transform: translateX(-150px) translateZ(-230px) rotateY(90deg);
        transform-origin: center left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-l-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-l-fwd".to_string(),
        r#"@keyframes slide-rotate-ver-l-fwd {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: center right
    }
    to {
        transform: translateX(-150px) translateZ(130px) rotateY(90deg);
        transform-origin: center right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-l-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-left".to_string(),
        r#"@keyframes slide-rotate-ver-left {
    0% {
        transform: translateX(0) rotateY(0)
    }
    to {
        transform: translateX(-150px) rotateY(90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-left;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-r-bck".to_string(),
        r#"@keyframes slide-rotate-ver-r-bck {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: center right
    }
    to {
        transform: translateX(150px) translateZ(-230px) rotateY(-90deg);
        transform-origin: center right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-r-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-r-fwd".to_string(),
        r#"@keyframes slide-rotate-ver-r-fwd {
    0% {
        transform: translateX(0) translateZ(0) rotateY(0);
        transform-origin: center left
    }
    to {
        transform: translateX(150px) translateZ(130px) rotateY(-90deg);
        transform-origin: center left
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-r-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-rotate-ver-right".to_string(),
        r#"@keyframes slide-rotate-ver-right {
    0% {
        transform: translateX(0) rotateY(0)
    }
    to {
        transform: translateX(150px) rotateY(-90deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-rotate-ver-right;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-tl".to_string(),
        r#"@keyframes slide-tl {
    0% {
        transform: translateY(0) translateX(0)
    }
    to {
        transform: translateY(-100px) translateX(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-top".to_string(),
        r#"@keyframes slide-top {
    0% {
        transform: translateY(0)
    }
    to {
        transform: translateY(-100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-top;
}
"#
        .to_string(),
    );
    map.insert(
        "slide-tr".to_string(),
        r#"@keyframes slide-tr {
    0% {
        transform: translateY(0) translateX(0)
    }
    to {
        transform: translateY(-100px) translateX(100px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slide-tr;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-in-diagonal-1".to_string(),
        r#"@keyframes slit-in-diagonal-1 {
    0% {
        transform: translateZ(-800px) rotate3d(1, 1, 0, 90deg);
        animation-timing-function: ease-in;
        opacity: 0
    }
    54% {
        transform: translateZ(-160px) rotate3d(1, 1, 0, 87deg);
        animation-timing-function: ease-in-out;
        opacity: 1
    }
    to {
        transform: translateZ(0) rotate3d(1, 1, 0, 0);
        animation-timing-function: ease-out
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-in-diagonal-1;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-in-diagonal-2".to_string(),
        r#"@keyframes slit-in-diagonal-2 {
    0% {
        transform: translateZ(-800px) rotate3d(-1, 1, 0, -90deg);
        animation-timing-function: ease-in;
        opacity: 0
    }
    54% {
        transform: translateZ(-160px) rotate3d(-1, 1, 0, -87deg);
        animation-timing-function: ease-in-out;
        opacity: 1
    }
    to {
        transform: translateZ(0) rotate3d(-1, 1, 0, 0);
        animation-timing-function: ease-out
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-in-diagonal-2;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-in-horizontal".to_string(),
        r#"@keyframes slit-in-horizontal {
    0% {
        transform: translateZ(-800px) rotateX(90deg);
        opacity: 0
    }
    54% {
        transform: translateZ(-160px) rotateX(87deg);
        opacity: 1
    }
    to {
        transform: translateZ(0) rotateX(0)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-in-horizontal;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-in-vertical".to_string(),
        r#"@keyframes slit-in-vertical {
    0% {
        transform: translateZ(-800px) rotateY(90deg);
        opacity: 0
    }
    54% {
        transform: translateZ(-160px) rotateY(87deg);
        opacity: 1
    }
    to {
        transform: translateZ(0) rotateY(0)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-in-vertical;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-out-diagonal-1".to_string(),
        r#"@keyframes slit-out-diagonal-1 {
    0% {
        transform: translateZ(0) rotate3d(1, 1, 0, 0);
        opacity: 1
    }
    54% {
        transform: translateZ(-160px) rotate3d(1, 1, 0, 87deg);
        opacity: 1
    }
    to {
        transform: translateZ(-800px) rotate3d(1, 1, 0, 90deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-out-diagonal-1;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-out-diagonal-2".to_string(),
        r#"@keyframes slit-out-diagonal-2 {
    0% {
        transform: translateZ(0) rotate3d(-1, 1, 0, 0);
        opacity: 1
    }
    54% {
        transform: translateZ(-160px) rotate3d(-1, 1, 0, -87deg);
        opacity: 1
    }
    to {
        transform: translateZ(-800px) rotate3d(-1, 1, 0, -90deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-out-diagonal-2;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-out-horizontal".to_string(),
        r#"@keyframes slit-out-horizontal {
    0% {
        transform: translateZ(0) rotateX(0);
        opacity: 1
    }
    54% {
        transform: translateZ(-160px) rotateX(87deg);
        opacity: 1
    }
    to {
        transform: translateZ(-800px) rotateX(90deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-out-horizontal;
}
"#
        .to_string(),
    );
    map.insert(
        "slit-out-vertical".to_string(),
        r#"@keyframes slit-out-vertical {
    0% {
        transform: translateZ(0) rotateY(0);
        opacity: 1
    }
    54% {
        transform: translateZ(-160px) rotateY(87deg);
        opacity: 1
    }
    to {
        transform: translateZ(-800px) rotateY(90deg);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: slit-out-vertical;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-bck".to_string(),
        r#"@keyframes swing-bottom-bck {
    0% {
        transform: rotateX(0);
        transform-origin: bottom
    }
    to {
        transform: rotateX(180deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-fwd".to_string(),
        r#"@keyframes swing-bottom-fwd {
    0% {
        transform: rotateX(0);
        transform-origin: bottom
    }
    to {
        transform: rotateX(-180deg);
        transform-origin: bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-left-bck".to_string(),
        r#"@keyframes swing-bottom-left-bck {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 0% 100%
    }
    to {
        transform: rotate3d(1, 1, 0, 180deg);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-left-fwd".to_string(),
        r#"@keyframes swing-bottom-left-fwd {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 0% 100%
    }
    to {
        transform: rotate3d(1, 1, 0, -180deg);
        transform-origin: 0% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-right-bck".to_string(),
        r#"@keyframes swing-bottom-right-bck {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 100% 100%
    }
    to {
        transform: rotate3d(-1, 1, 0, -180deg);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-bottom-right-fwd".to_string(),
        r#"@keyframes swing-bottom-right-fwd {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 100% 100%
    }
    to {
        transform: rotate3d(-1, 1, 0, 180deg);
        transform-origin: 100% 100%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-bottom-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-bottom-bck".to_string(),
        r#"@keyframes swing-in-bottom-bck {
    0% {
        transform: rotateX(-70deg);
        transform-origin: bottom;
        opacity: 0
    }
    to {
        transform: rotateX(0);
        transform-origin: bottom;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-bottom-fwd".to_string(),
        r#"@keyframes swing-in-bottom-fwd {
    0% {
        transform: rotateX(100deg);
        transform-origin: bottom;
        opacity: 0
    }
    to {
        transform: rotateX(0);
        transform-origin: bottom;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-left-bck".to_string(),
        r#"@keyframes swing-in-left-bck {
    0% {
        transform: rotateY(-70deg);
        transform-origin: left;
        opacity: 0
    }
    to {
        transform: rotateY(0);
        transform-origin: left;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-left-fwd".to_string(),
        r#"@keyframes swing-in-left-fwd {
    0% {
        transform: rotateY(100deg);
        transform-origin: left;
        opacity: 0
    }
    to {
        transform: rotateY(0);
        transform-origin: left;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-right-bck".to_string(),
        r#"@keyframes swing-in-right-bck {
    0% {
        transform: rotateY(70deg);
        transform-origin: right;
        opacity: 0
    }
    to {
        transform: rotateY(0);
        transform-origin: right;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-right-fwd".to_string(),
        r#"@keyframes swing-in-right-fwd {
    0% {
        transform: rotateY(-100deg);
        transform-origin: right;
        opacity: 0
    }
    to {
        transform: rotateY(0);
        transform-origin: right;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-top-bck".to_string(),
        r#"@keyframes swing-in-top-bck {
    0% {
        transform: rotateX(70deg);
        transform-origin: top;
        opacity: 0
    }
    to {
        transform: rotateX(0deg);
        transform-origin: top;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-in-top-fwd".to_string(),
        r#"@keyframes swing-in-top-fwd {
    0% {
        transform: rotateX(-100deg);
        transform-origin: top;
        opacity: 0
    }
    to {
        transform: rotateX(0deg);
        transform-origin: top;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-in-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-left-bck".to_string(),
        r#"@keyframes swing-left-bck {
    0% {
        transform: rotateY(0);
        transform-origin: left bottom
    }
    to {
        transform: rotateY(180deg);
        transform-origin: left bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-left-fwd".to_string(),
        r#"@keyframes swing-left-fwd {
    0% {
        transform: rotateY(0);
        transform-origin: left bottom
    }
    to {
        transform: rotateY(-180deg);
        transform-origin: left bottom
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-bottom-bck".to_string(),
        r#"@keyframes swing-out-bottom-bck {
    0% {
        transform: rotateX(0);
        transform-origin: bottom;
        opacity: 1
    }
    to {
        transform: rotateX(100deg);
        transform-origin: bottom;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-bottom-fwd".to_string(),
        r#"@keyframes swing-out-bottom-fwd {
    0% {
        transform: rotateX(0);
        transform-origin: bottom;
        opacity: 1
    }
    to {
        transform: rotateX(-70deg);
        transform-origin: bottom;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-left-bck".to_string(),
        r#"@keyframes swing-out-left-bck {
    0% {
        transform: rotateY(0);
        transform-origin: left;
        opacity: 1
    }
    to {
        transform: rotateY(100deg);
        transform-origin: left;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-left-fwd".to_string(),
        r#"@keyframes swing-out-left-fwd {
    0% {
        transform: rotateY(0);
        transform-origin: left;
        opacity: 1
    }
    to {
        transform: rotateY(-70deg);
        transform-origin: left;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-right-bck".to_string(),
        r#"@keyframes swing-out-right-bck {
    0% {
        transform: rotateY(0);
        transform-origin: right;
        opacity: 1
    }
    to {
        transform: rotateY(-100deg);
        transform-origin: right;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-right-fwd".to_string(),
        r#"@keyframes swing-out-right-fwd {
    0% {
        transform: rotateY(0);
        transform-origin: right;
        opacity: 1
    }
    to {
        transform: rotateY(70deg);
        transform-origin: right;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-top-bck".to_string(),
        r#"@keyframes swing-out-top-bck {
    0% {
        transform: rotateX(0deg);
        transform-origin: top;
        opacity: 1
    }
    to {
        transform: rotateX(-100deg);
        transform-origin: top;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-out-top-fwd".to_string(),
        r#"@keyframes swing-out-top-fwd {
    0% {
        transform: rotateX(0deg);
        transform-origin: top;
        opacity: 1
    }
    to {
        transform: rotateX(70deg);
        transform-origin: top;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-out-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-right-bck".to_string(),
        r#"@keyframes swing-right-bck {
    0% {
        transform: rotateY(0);
        transform-origin: right
    }
    to {
        transform: rotateY(-180deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-right-fwd".to_string(),
        r#"@keyframes swing-right-fwd {
    0% {
        transform: rotateY(0);
        transform-origin: right
    }
    to {
        transform: rotateY(180deg);
        transform-origin: right
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-bck".to_string(),
        r#"@keyframes swing-top-bck {
    0% {
        transform: rotateX(0);
        transform-origin: top
    }
    to {
        transform: rotateX(-180deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-fwd".to_string(),
        r#"@keyframes swing-top-fwd {
    0% {
        transform: rotateX(0);
        transform-origin: top
    }
    to {
        transform: rotateX(180deg);
        transform-origin: top
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-left-bck".to_string(),
        r#"@keyframes swing-top-left-bck {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 0% 0%
    }
    to {
        transform: rotate3d(-1, 1, 0, 180deg);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-left-fwd".to_string(),
        r#"@keyframes swing-top-left-fwd {
    0% {
        transform: rotate3d(-1, 1, 0, 0deg);
        transform-origin: 0% 0%
    }
    to {
        transform: rotate3d(-1, 1, 0, -180deg);
        transform-origin: 0% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-right-bck".to_string(),
        r#"@keyframes swing-top-right-bck {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 100% 0%
    }
    to {
        transform: rotate3d(1, 1, 0, -180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swing-top-right-fwd".to_string(),
        r#"@keyframes swing-top-right-fwd {
    0% {
        transform: rotate3d(1, 1, 0, 0deg);
        transform-origin: 100% 0%
    }
    to {
        transform: rotate3d(1, 1, 0, 180deg);
        transform-origin: 100% 0%
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swing-top-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swing".to_string(),
        r#"@keyframes swing {
  20% {
    transform: rotate3d(0, 0, 1, 15deg);
  }

  40% {
    transform: rotate3d(0, 0, 1, -10deg);
  }

  60% {
    transform: rotate3d(0, 0, 1, 5deg);
  }

  80% {
    transform: rotate3d(0, 0, 1, -5deg);
  }

  to {
    transform: rotate3d(0, 0, 1, 0deg);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  transform-origin: top center;
  animation-name: swing;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-bck".to_string(),
        r#"@keyframes swirl-in-bck {
    0% {
        transform: rotate(540deg) scale(5);
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-bl-bck".to_string(),
        r#"@keyframes swirl-in-bl-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 0 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-bl-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-bl-fwd".to_string(),
        r#"@keyframes swirl-in-bl-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-bl-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-bottom-bck".to_string(),
        r#"@keyframes swirl-in-bottom-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 50% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-bottom-fwd".to_string(),
        r#"@keyframes swirl-in-bottom-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-br-bck".to_string(),
        r#"@keyframes swirl-in-br-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-br-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-br-fwd".to_string(),
        r#"@keyframes swirl-in-br-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 100%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-br-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-fwd".to_string(),
        r#"@keyframes swirl-in-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-left-bck".to_string(),
        r#"@keyframes swirl-in-left-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 0 50%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-left-fwd".to_string(),
        r#"@keyframes swirl-in-left-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 50%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-right-bck".to_string(),
        r#"@keyframes swirl-in-right-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 50%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-right-fwd".to_string(),
        r#"@keyframes swirl-in-right-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 50%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-tl-bck".to_string(),
        r#"@keyframes swirl-in-tl-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 0 0;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-tl-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-tl-fwd".to_string(),
        r#"@keyframes swirl-in-tl-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 0;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 0 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-tl-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-top-bck".to_string(),
        r#"@keyframes swirl-in-top-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 50% 0;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 50% 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-top-fwd".to_string(),
        r#"@keyframes swirl-in-top-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 50% 0;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 50% 0;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-tr-bck".to_string(),
        r#"@keyframes swirl-in-tr-bck {
    0% {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 0%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-tr-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-in-tr-fwd".to_string(),
        r#"@keyframes swirl-in-tr-fwd {
    0% {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 0%;
        opacity: 0
    }
    to {
        transform: rotate(0) scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-in-tr-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-bck".to_string(),
        r#"@keyframes swirl-out-bck {
    0% {
        transform: rotate(0) scale(1);
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-bl-bck".to_string(),
        r#"@keyframes swirl-out-bl-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 100%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-bl-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-bl-fwd".to_string(),
        r#"@keyframes swirl-out-bl-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 100%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 0 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-bl-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-bottom-bck".to_string(),
        r#"@keyframes swirl-out-bottom-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 50% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-bottom-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-bottom-fwd".to_string(),
        r#"@keyframes swirl-out-bottom-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 50% 100%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 50% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-bottom-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-br-bck".to_string(),
        r#"@keyframes swirl-out-br-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-br-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-br-fwd".to_string(),
        r#"@keyframes swirl-out-br-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 100%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 100%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-br-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-fwd".to_string(),
        r#"@keyframes swirl-out-fwd {
    0% {
        transform: rotate(0) scale(1);
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-left-bck".to_string(),
        r#"@keyframes swirl-out-left-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 50%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-left-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-left-fwd".to_string(),
        r#"@keyframes swirl-out-left-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 50%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 0 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-left-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-right-bck".to_string(),
        r#"@keyframes swirl-out-right-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-right-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-right-fwd".to_string(),
        r#"@keyframes swirl-out-right-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 50%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 50%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-right-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-tl-bck".to_string(),
        r#"@keyframes swirl-out-tl-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 0 0;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-tl-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-tl-fwd".to_string(),
        r#"@keyframes swirl-out-tl-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 0 0;
        opacity: 1
    }
    to {
        transform: rotate(720deg) scale(5);
        transform-origin: 0 0;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-tl-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-top-bck".to_string(),
        r#"@keyframes swirl-out-top-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 50% 0%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 50% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-top-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-top-fwd".to_string(),
        r#"@keyframes swirl-out-top-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 50% 0%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 50% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-top-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-tr-bck".to_string(),
        r#"@keyframes swirl-out-tr-bck {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: rotate(-540deg) scale(0);
        transform-origin: 100% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-tr-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "swirl-out-tr-fwd".to_string(),
        r#"@keyframes swirl-out-tr-fwd {
    0% {
        transform: rotate(0) scale(1);
        transform-origin: 100% 0%;
        opacity: 1
    }
    to {
        transform: rotate(540deg) scale(5);
        transform-origin: 100% 0%;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: swirl-out-tr-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "tada".to_string(),
        r#"@keyframes tada {
  from {
    transform: scale3d(1, 1, 1);
  }

  10%,
  20% {
    transform: scale3d(0.9, 0.9, 0.9) rotate3d(0, 0, 1, -3deg);
  }

  30%,
  50%,
  70%,
  90% {
    transform: scale3d(1.1, 1.1, 1.1) rotate3d(0, 0, 1, 3deg);
  }

  40%,
  60%,
  80% {
    transform: scale3d(1.1, 1.1, 1.1) rotate3d(0, 0, 1, -3deg);
  }

  to {
    transform: scale3d(1, 1, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tada;
}
"#
        .to_string(),
    );
    map.insert(
        "text-blur-out".to_string(),
        r#"@keyframes text-blur-out {
    0% {
        filter: blur(.01)
    }
    to {
        filter: blur(12px) opacity(0%)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-blur-out;
}
"#
        .to_string(),
    );
    map.insert("text-flicker-in-glow".to_string(), r#"@keyframes text-flicker-in-glow {
    0% {
        opacity: 0
    }
    10%,
    10.2%,
    20%,
    20.6%,
    30%,
    30.6%,
    45%,
    55.1%,
    57%,
    60.1%,
    65%,
    75.1%,
    77%,
    85.1%,
    86% {
        opacity: 0;
        text-shadow: none
    }
    10.1% {
        opacity: 1;
        text-shadow: none
    }
    20.1% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .25)
    }
    30.1%,
    30.5%,
    45.1%,
    50%,
    55% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .45), 0 0 60px rgba(255, 255, 255, .25)
    }
    57.1%,
    60% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35)
    }
    65.1%,
    75% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35), 0 0 100px rgba(255, 255, 255, .1)
    }
    77.1%,
    85% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .4), 0 0 110px rgba(255, 255, 255, .2), 0 0 100px rgba(255, 255, 255, .1)
    }
    86.1%,
    to {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .6), 0 0 60px rgba(255, 255, 255, .45), 0 0 110px rgba(255, 255, 255, .25), 0 0 100px rgba(255, 255, 255, .1)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-flicker-in-glow;
}
"#.to_string());
    map.insert("text-flicker-out-glow".to_string(), r#"@keyframes text-flicker-out-glow {
    0%,
    13.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .6), 0 0 60px rgba(255, 255, 255, .45), 0 0 110px rgba(255, 255, 255, .25), 0 0 100px rgba(255, 255, 255, .1)
    }
    14%,
    14.9%,
    23%,
    24.9%,
    35%,
    39.9%,
    43%,
    44.9%,
    55%,
    69.4%,
    70%,
    79.4%,
    80%,
    89.8%,
    90% {
        opacity: 0;
        text-shadow: none
    }
    15%,
    22.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .4), 0 0 110px rgba(255, 255, 255, .2), 0 0 100px rgba(255, 255, 255, .1)
    }
    25%,
    34.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35), 0 0 100px rgba(255, 255, 255, .1)
    }
    40%,
    42.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .55), 0 0 60px rgba(255, 255, 255, .35)
    }
    45%,
    50%,
    54.9%,
    69.5%,
    69.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .45), 0 0 60px rgba(255, 255, 255, .25)
    }
    79.9% {
        opacity: 1;
        text-shadow: 0 0 30px rgba(255, 255, 255, .25)
    }
    89.9% {
        opacity: 1;
        text-shadow: none
    }
    to {
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-flicker-out-glow;
}
"#.to_string());
    map.insert(
        "text-focus-in".to_string(),
        r#"@keyframes text-focus-in {
    0% {
        filter: blur(12px);
        opacity: 0
    }
    to {
        filter: blur(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-focus-in;
}
"#
        .to_string(),
    );
    map.insert("text-pop-up-bl".to_string(), r#"@keyframes text-pop-up-bl {
    0% {
        transform: translateY(0) translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(50px) translateX(-50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-bl;
}
"#.to_string());
    map.insert("text-pop-up-bottom".to_string(), r#"@keyframes text-pop-up-bottom {
    0% {
        transform: translateY(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-bottom;
}
"#.to_string());
    map.insert("text-pop-up-br".to_string(), r#"@keyframes text-pop-up-br {
    0% {
        transform: translateY(0) translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(50px) translateX(50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-br;
}
"#.to_string());
    map.insert("text-pop-up-left".to_string(), r#"@keyframes text-pop-up-left {
    0% {
        transform: translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateX(-50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-left;
}
"#.to_string());
    map.insert("text-pop-up-right".to_string(), r#"@keyframes text-pop-up-right {
    0% {
        transform: translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateX(50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-right;
}
"#.to_string());
    map.insert("text-pop-up-tl".to_string(), r#"@keyframes text-pop-up-tl {
    0% {
        transform: translateY(0) translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(-50px) translateX(-50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-tl;
}
"#.to_string());
    map.insert("text-pop-up-top".to_string(), r#"@keyframes text-pop-up-top {
    0% {
        transform: translateY(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(-50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-top;
}
"#.to_string());
    map.insert("text-pop-up-tr".to_string(), r#"@keyframes text-pop-up-tr {
    0% {
        transform: translateY(0) translateX(0);
        transform-origin: 50% 50%;
        text-shadow: none
    }
    to {
        transform: translateY(-50px) translateX(50px);
        transform-origin: 50% 50%;
        text-shadow: 0 1px 0 #ccc, 0 2px 0 #ccc, 0 3px 0 #ccc, 0 4px 0 #ccc, 0 5px 0 #ccc, 0 6px 0 #ccc, 0 7px 0 #ccc, 0 8px 0 #ccc, 0 9px 0 #ccc, 0 50px 30px rgba(0, 0, 0, .3)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-pop-up-tr;
}
"#.to_string());
    map.insert(
        "text-shadow-drop-bl".to_string(),
        r#"@keyframes text-shadow-drop-bl {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: -6px 6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-bl;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-bottom".to_string(),
        r#"@keyframes text-shadow-drop-bottom {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 0 6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-br".to_string(),
        r#"@keyframes text-shadow-drop-br {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 6px 6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-br;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-center".to_string(),
        r#"@keyframes text-shadow-drop-center {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 0 0 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-center;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-left".to_string(),
        r#"@keyframes text-shadow-drop-left {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: -6px 0 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-left;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-right".to_string(),
        r#"@keyframes text-shadow-drop-right {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 6px 0 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-right;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-tl".to_string(),
        r#"@keyframes text-shadow-drop-tl {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: -6px -6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-tl;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-top".to_string(),
        r#"@keyframes text-shadow-drop-top {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 0 -6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-top;
}
"#
        .to_string(),
    );
    map.insert(
        "text-shadow-drop-tr".to_string(),
        r#"@keyframes text-shadow-drop-tr {
    0% {
        text-shadow: 0 0 0 transparent
    }
    to {
        text-shadow: 6px -6px 18px rgba(0, 0, 0, .35)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-drop-tr;
}
"#
        .to_string(),
    );
    map.insert("text-shadow-pop-bl".to_string(), r#"@keyframes text-shadow-pop-bl {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0) translateY(0)
    }
    to {
        text-shadow: -1px 1px #555, -2px 2px #555, -3px 3px #555, -4px 4px #555, -5px 5px #555, -6px 6px #555, -7px 7px #555, -8px 8px #555;
        transform: translateX(8px) translateY(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-bl;
}
"#.to_string());
    map.insert("text-shadow-pop-bottom".to_string(), r#"@keyframes text-shadow-pop-bottom {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateY(0)
    }
    to {
        text-shadow: 0 1px #555, 0 2px #555, 0 3px #555, 0 4px #555, 0 5px #555, 0 6px #555, 0 7px #555, 0 8px #555;
        transform: translateY(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-bottom;
}
"#.to_string());
    map.insert("text-shadow-pop-br".to_string(), r#"@keyframes text-shadow-pop-br {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0) translateY(0)
    }
    to {
        text-shadow: 1px 1px #555, 2px 2px #555, 3px 3px #555, 4px 4px #555, 5px 5px #555, 6px 6px #555, 7px 7px #555, 8px 8px #555;
        transform: translateX(-8px) translateY(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-br;
}
"#.to_string());
    map.insert("text-shadow-pop-left".to_string(), r#"@keyframes text-shadow-pop-left {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0)
    }
    to {
        text-shadow: -1px 0 #555, -2px 0 #555, -3px 0 #555, -4px 0 #555, -5px 0 #555, -6px 0 #555, -7px 0 #555, -8px 0 #555;
        transform: translateX(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-left;
}
"#.to_string());
    map.insert("text-shadow-pop-right".to_string(), r#"@keyframes text-shadow-pop-right {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0)
    }
    to {
        text-shadow: 1px 0 #555, 2px 0 #555, 3px 0 #555, 4px 0 #555, 5px 0 #555, 6px 0 #555, 7px 0 #555, 8px 0 #555;
        transform: translateX(-8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-right;
}
"#.to_string());
    map.insert("text-shadow-pop-tl".to_string(), r#"@keyframes text-shadow-pop-tl {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0) translateY(0)
    }
    to {
        text-shadow: -1px -1px #555, -2px -2px #555, -3px -3px #555, -4px -4px #555, -5px -5px #555, -6px -6px #555, -7px -7px #555, -8px -8px #555;
        transform: translateX(8px) translateY(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-tl;
}
"#.to_string());
    map.insert("text-shadow-pop-top".to_string(), r#"@keyframes text-shadow-pop-top {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateY(0)
    }
    to {
        text-shadow: 0 -1px #555, 0 -2px #555, 0 -3px #555, 0 -4px #555, 0 -5px #555, 0 -6px #555, 0 -7px #555, 0 -8px #555;
        transform: translateY(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-top;
}
"#.to_string());
    map.insert("text-shadow-pop-tr".to_string(), r#"@keyframes text-shadow-pop-tr {
    0% {
        text-shadow: 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555, 0 0 #555;
        transform: translateX(0) translateY(0)
    }
    to {
        text-shadow: 1px -1px #555, 2px -2px #555, 3px -3px #555, 4px -4px #555, 5px -5px #555, 6px -6px #555, 7px -7px #555, 8px -8px #555;
        transform: translateX(-8px) translateY(8px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: text-shadow-pop-tr;
}
"#.to_string());
    map.insert(
        "tracking-in-contract-bck-bottom".to_string(),
        r#"@keyframes tracking-in-contract-bck-bottom {
    0% {
        letter-spacing: 1em;
        transform: translateZ(400px) translateY(300px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-contract-bck-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-contract-bck-top".to_string(),
        r#"@keyframes tracking-in-contract-bck-top {
    0% {
        letter-spacing: 1em;
        transform: translateZ(400px) translateY(-300px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-contract-bck-top;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-contract-bck".to_string(),
        r#"@keyframes tracking-in-contract-bck {
    0% {
        letter-spacing: 1em;
        transform: translateZ(400px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-contract-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-contract".to_string(),
        r#"@keyframes tracking-in-contract {
    0% {
        letter-spacing: 1em;
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        letter-spacing: normal;
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-contract;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-expand-fwd-bottom".to_string(),
        r#"@keyframes tracking-in-expand-fwd-bottom {
    0% {
        letter-spacing: -.5em;
        transform: translateZ(-700px) translateY(500px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-expand-fwd-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-expand-fwd-top".to_string(),
        r#"@keyframes tracking-in-expand-fwd-top {
    0% {
        letter-spacing: -.5em;
        transform: translateZ(-700px) translateY(-500px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-expand-fwd-top;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-expand-fwd".to_string(),
        r#"@keyframes tracking-in-expand-fwd {
    0% {
        letter-spacing: -.5em;
        transform: translateZ(-700px);
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        transform: translateZ(0);
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-expand-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-in-expand".to_string(),
        r#"@keyframes tracking-in-expand {
    0% {
        letter-spacing: -.5em;
        opacity: 0
    }
    40% {
        opacity: .6
    }
    to {
        opacity: 1
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-in-expand;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-contract-bck-bottom".to_string(),
        r#"@keyframes tracking-out-contract-bck-bottom {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    60% {
        opacity: 1
    }
    to {
        letter-spacing: -.5em;
        transform: translateZ(-500px) translateY(300px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-contract-bck-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-contract-bck-top".to_string(),
        r#"@keyframes tracking-out-contract-bck-top {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    60% {
        opacity: 1
    }
    to {
        letter-spacing: -.5em;
        transform: translateZ(-500px) translateY(-300px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-contract-bck-top;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-contract-bck".to_string(),
        r#"@keyframes tracking-out-contract-bck {
    0% {
        transform: translateZ(0);
        opacity: 1
    }
    60% {
        opacity: 1
    }
    to {
        letter-spacing: -.5em;
        transform: translateZ(-500px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-contract-bck;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-contract".to_string(),
        r#"@keyframes tracking-out-contract {
    0%,
    50% {
        opacity: 1
    }
    to {
        letter-spacing: -.5em;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-contract;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-expand-fwd-bottom".to_string(),
        r#"@keyframes tracking-out-expand-fwd-bottom {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    60% {
        opacity: .8
    }
    to {
        letter-spacing: 1em;
        transform: translateZ(300px) translateY(200px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-expand-fwd-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-expand-fwd-top".to_string(),
        r#"@keyframes tracking-out-expand-fwd-top {
    0% {
        transform: translateZ(0) translateY(0);
        opacity: 1
    }
    60% {
        opacity: .8
    }
    to {
        letter-spacing: 1em;
        transform: translateZ(300px) translateY(-200px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-expand-fwd-top;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-expand-fwd".to_string(),
        r#"@keyframes tracking-out-expand-fwd {
    0% {
        transform: translateZ(0);
        opacity: 1
    }
    60% {
        opacity: .8
    }
    to {
        letter-spacing: 1em;
        transform: translateZ(300px);
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-expand-fwd;
}
"#
        .to_string(),
    );
    map.insert(
        "tracking-out-expand".to_string(),
        r#"@keyframes tracking-out-expand {
    0% {
        opacity: 1
    }
    60% {
        opacity: .8
    }
    to {
        letter-spacing: 1em;
        opacity: 0
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: tracking-out-expand;
}
"#
        .to_string(),
    );
    map.insert(
        "vibrate-1".to_string(),
        r#"@keyframes vibrate-1 {
    0%,
    to {
        transform: translate(0)
    }
    20% {
        transform: translate(-2px, 2px)
    }
    40% {
        transform: translate(-2px, -2px)
    }
    60% {
        transform: translate(2px, 2px)
    }
    80% {
        transform: translate(2px, -2px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: vibrate-1;
}
"#
        .to_string(),
    );
    map.insert(
        "vibrate-2".to_string(),
        r#"@keyframes vibrate-2 {
    0%,
    to {
        transform: translate(0)
    }
    20% {
        transform: translate(2px, -2px)
    }
    40% {
        transform: translate(2px, 2px)
    }
    60% {
        transform: translate(-2px, 2px)
    }
    80% {
        transform: translate(-2px, -2px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: vibrate-2;
}
"#
        .to_string(),
    );
    map.insert(
        "vibrate-3".to_string(),
        r#"@keyframes vibrate-3 {
    0%,
    to {
        transform: translate(0)
    }
    10%,
    50%,
    80% {
        transform: translate(-2px, -2px)
    }
    20%,
    60%,
    90% {
        transform: translate(2px, -2px)
    }
    30%,
    70% {
        transform: translate(-2px, 2px)
    }
    40% {
        transform: translate(2px, 2px)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: vibrate-3;
}
"#
        .to_string(),
    );
    map.insert(
        "wobble-hor-bottom".to_string(),
        r#"@keyframes wobble-hor-bottom {
    0%,
    to {
        transform: translateX(0%);
        transform-origin: 50% 50%
    }
    15% {
        transform: translateX(-30px) rotate(-6deg)
    }
    30% {
        transform: translateX(15px) rotate(6deg)
    }
    45% {
        transform: translateX(-15px) rotate(-3.6deg)
    }
    60% {
        transform: translateX(9px) rotate(2.4deg)
    }
    75% {
        transform: translateX(-6px) rotate(-1.2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: wobble-hor-bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "wobble-hor-top".to_string(),
        r#"@keyframes wobble-hor-top {
    0%,
    to {
        transform: translateX(0%);
        transform-origin: 50% 50%
    }
    15% {
        transform: translateX(-30px) rotate(6deg)
    }
    30% {
        transform: translateX(15px) rotate(-6deg)
    }
    45% {
        transform: translateX(-15px) rotate(3.6deg)
    }
    60% {
        transform: translateX(9px) rotate(-2.4deg)
    }
    75% {
        transform: translateX(-6px) rotate(1.2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: wobble-hor-top;
}
"#
        .to_string(),
    );
    map.insert(
        "wobble-ver-left".to_string(),
        r#"@keyframes wobble-ver-left {
    0%,
    to {
        transform: translateY(0) rotate(0);
        transform-origin: 50% 50%
    }
    15% {
        transform: translateY(-30px) rotate(-6deg)
    }
    30% {
        transform: translateY(15px) rotate(6deg)
    }
    45% {
        transform: translateY(-15px) rotate(-3.6deg)
    }
    60% {
        transform: translateY(9px) rotate(2.4deg)
    }
    75% {
        transform: translateY(-6px) rotate(-1.2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: wobble-ver-left;
}
"#
        .to_string(),
    );
    map.insert(
        "wobble-ver-right".to_string(),
        r#"@keyframes wobble-ver-right {
    0%,
    to {
        transform: translateY(0) rotate(0);
        transform-origin: 50% 50%
    }
    15% {
        transform: translateY(-30px) rotate(6deg)
    }
    30% {
        transform: translateY(15px) rotate(-6deg)
    }
    45% {
        transform: translateY(-15px) rotate(3.6deg)
    }
    60% {
        transform: translateY(9px) rotate(-2.4deg)
    }
    75% {
        transform: translateY(-6px) rotate(1.2deg)
    }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: wobble-ver-right;
}
"#
        .to_string(),
    );
    map.insert(
        "wobble".to_string(),
        r#"@keyframes wobble {
  from {
    transform: translate3d(0, 0, 0);
  }

  15% {
    transform: translate3d(-25%, 0, 0) rotate3d(0, 0, 1, -5deg);
  }

  30% {
    transform: translate3d(20%, 0, 0) rotate3d(0, 0, 1, 3deg);
  }

  45% {
    transform: translate3d(-15%, 0, 0) rotate3d(0, 0, 1, -3deg);
  }

  60% {
    transform: translate3d(10%, 0, 0) rotate3d(0, 0, 1, 2deg);
  }

  75% {
    transform: translate3d(-5%, 0, 0) rotate3d(0, 0, 1, -1deg);
  }

  to {
    transform: translate3d(0, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: wobble;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-in-down".to_string(),
        r#"@keyframes zoom-in-down {
  from {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(0, -1000px, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  60% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(0, 60px, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-in-down;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-in-left".to_string(),
        r#"@keyframes zoom-in-left {
  from {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(-1000px, 0, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  60% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(10px, 0, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-in-left;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-in-right".to_string(),
        r#"@keyframes zoom-in-right {
  from {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(1000px, 0, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  60% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(-10px, 0, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-in-right;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-in-up".to_string(),
        r#"@keyframes zoom-in-up {
  from {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(0, 1000px, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  60% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(0, -60px, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-in-up;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-in".to_string(),
        r#"@keyframes zoom-in {
  from {
    opacity: 0;
    transform: scale3d(0.3, 0.3, 0.3);
  }

  50% {
    opacity: 1;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-in;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-out-down".to_string(),
        r#"@keyframes zoom-out-down {
  40% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(0, -60px, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  to {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(0, 2000px, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-out-down;
  transform-origin: center bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-out-left".to_string(),
        r#"@keyframes zoom-out-left {
  40% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(42px, 0, 0);
  }

  to {
    opacity: 0;
    transform: scale(0.1) translate3d(-2000px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-out-left;
  transform-origin: left center;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-out-right".to_string(),
        r#"@keyframes zoom-out-right {
  40% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(-42px, 0, 0);
  }

  to {
    opacity: 0;
    transform: scale(0.1) translate3d(2000px, 0, 0);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-out-right;
  transform-origin: right center;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-out-up".to_string(),
        r#"@keyframes zoom-out-up {
  40% {
    opacity: 1;
    transform: scale3d(0.475, 0.475, 0.475) translate3d(0, 60px, 0);
    animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  to {
    opacity: 0;
    transform: scale3d(0.1, 0.1, 0.1) translate3d(0, -2000px, 0);
    animation-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-out-up;
  transform-origin: center bottom;
}
"#
        .to_string(),
    );
    map.insert(
        "zoom-out".to_string(),
        r#"@keyframes zoom-out {
  from {
    opacity: 1;
  }

  50% {
    opacity: 0;
    transform: scale3d(0.3, 0.3, 0.3);
  }

  to {
    opacity: 0;
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: zoom-out;
}
"#
        .to_string(),
    );
    map
});
