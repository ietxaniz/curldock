import { useAppDispatch, useAppSelector } from "./hooks";
import { setHeaderHeight } from '../slices/layoutSlice';

export const useHeaderHeight = () => useAppSelector(state => state.layout.headerHeight);
export const useSetHeaderHeight = () => {
  const dispatch = useAppDispatch();
  return (height:number) => dispatch(setHeaderHeight(height));
};
