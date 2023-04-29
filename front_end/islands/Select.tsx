import { JSX } from "preact";
import { IS_BROWSER } from "$fresh/runtime.ts";
import { Muscle, WeightUnit, WorkoutType } from "@/types/workouts.ts";

type IterableEnum = typeof WorkoutType | typeof Muscle | typeof WeightUnit;

type SelectElementAttribute = JSX.HTMLAttributes<HTMLSelectElement> & {
  "data-options": IterableEnum;
};

export default function Select(props: SelectElementAttribute) {
  function renderOptions(): JSX.Element[] {
    const arr: JSX.Element[] = [];

    for (const child in props["data-options"]) {
      arr.push(<option class="p-1" value={child}>{child}</option>);
    }
    return arr;
  }

  return (
    <select
      {...props}
      disabled={!IS_BROWSER || props.disabled}
      class={`px-3 mx-3 max-w-md py-2 bg-white rounded border(gray-500 2) disabled:(opacity-50 cursor-not-allowed) ${
        props.class ?? ""
      }`}
    >
      {renderOptions()}
    </select>
  );
}
