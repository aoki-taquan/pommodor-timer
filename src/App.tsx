import { Button, ColorModeContext, DarkMode, Flex, Text } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

import { extendTheme, type ThemeConfig } from '@chakra-ui/react'
import { ChakraProvider, useColorMode } from '@chakra-ui/react'

// 2. Add your color mode config
const config: ThemeConfig = {
  initialColorMode: 'dark',
  useSystemColorMode: false,
}

// 3. extend the theme
const theme = extendTheme({ config })


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
    !timerStart ? invoke("restart_timer") : invoke("pause_timer");
    // emit_masseage(!timerStart ? "restart" : "pause");
  };


  function stert_timer(set_time_second: number) {
    invoke("start_timer", { setTimeSecond: set_time_second * 60 });
    invoke("chenge_now_time_longer", { newTimeLonger: set_time_second * 60 });

  }


  // Core側から値を受け取る
  useEffect(() => {
    theme;


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
        {/* いい感じに時間が表記されるようにする */}
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
                setTimerStart(true);
                stert_timer(value);
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