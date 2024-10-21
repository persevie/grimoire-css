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
