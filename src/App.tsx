import { Button, Flex, Text } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { ask } from "@tauri-apps/api/dialog";
import { listen } from '@tauri-apps/api/event'
import { emit } from '@tauri-apps/api/event'



function App() {
  //おまじない
  const [timerStart, setTimerStart] = useState(false);
  const [time, setTime] = useState(0);

  //使うボタンの定義
  const buttons = [
    {
      value: 5,
      display: "5 minutes",
    },
    {
      value: 25,
      display: "25 minutes",
    },
  ];

  //startとpuseの切り替え
  const toggleTimer = () => {
    setTimerStart(!timerStart);
    emit_masseage(!timerStart ? "restart" : "pause");
  };

  function emit_masseage(masseage: string) {
    emit("event-name", masseage)
  }

  // Core側から値を受け取る
  useEffect(() => {
    let unlisten: any;
    async function f() {
      unlisten = await listen('now-remining-time', (event) => {
        if (typeof event.payload === "number") {
          setTime(event.payload);

        }
      });
    }
    f();

    async function g() {
      unlisten = await listen('is_runing', (event) => {
        if (typeof event.payload === "boolean") {
          setTimerStart(event.payload);
        }
      });
    }
    g();
    return () => {
      f();
    }
  }, [])

  //描画する部分
  return (
    <div className="App" style={{ height: "100%" }}>
      <Flex
        background="gray.700"
        height="100%"
        alignItems="center"
        flexDirection="column"
      >
        <Text color="white" fontWeight="bold" marginTop="20" fontSize="35">
          Pomodoro Timer
        </Text>
        <Text fontWeight="bold" fontSize="7xl" color="white">
          {`${Math.floor(time / 60) < 10
            ? `0${Math.floor(time / 60)}`
            : `${Math.floor(time / 60)}`
            }:${time % 60 < 10 ? `0${time % 60}` : time % 60}`}
        </Text>
        <Flex>
          <Button
            width="7rem"
            background="tomato"
            color="white"
            onClick={toggleTimer}
          >
            {!timerStart ? "Start" : "Pause"}
          </Button>
        </Flex>
        <Flex marginTop={10}>
          {buttons.map(({ value, display }) => (
            <Button
              marginX={4}
              background="green.300"
              color="white"
              onClick={() => {
                setTimerStart(false);
                emit_masseage("start_" + value);
              }}
            >
              {display}
            </Button>
          ))}
        </Flex>
      </Flex>
    </div>
  );
}

export default App;