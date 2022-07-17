import { action } from '@storybook/addon-actions';
import HelloWorld from './HelloWorld.svelte';

export default {
  title: 'HelloWorld',
  component: HelloWorld,
};

export const Text = () => ({
  Component: HelloWorld,
  props: { }
});
