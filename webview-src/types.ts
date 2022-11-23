export type Area = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type Options = {
  path?: string;
  capture?: RequireAtLeastOne<{
    transparentBackground: boolean;
    highlighted: boolean;
  }>;
};

type RequireAtLeastOne<T, Keys extends keyof T = keyof T> = Pick<
  T,
  Exclude<keyof T, Keys>
> &
  {
    [K in Keys]-?: Required<Pick<T, K>> & Partial<Pick<T, Exclude<Keys, K>>>;
  }[Keys];