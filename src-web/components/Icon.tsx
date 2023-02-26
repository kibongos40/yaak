import {
  ArchiveIcon,
  CameraIcon,
  CheckIcon,
  GearIcon,
  HomeIcon,
  MoonIcon,
  PaperPlaneIcon,
  QuestionMarkIcon,
  SunIcon,
  TriangleDownIcon,
  UpdateIcon,
} from '@radix-ui/react-icons';
import classnames from 'classnames';
import { NamedExoticComponent } from 'react';

type IconName =
  | 'archive'
  | 'home'
  | 'camera'
  | 'gear'
  | 'triangle-down'
  | 'paper-plane'
  | 'update'
  | 'question'
  | 'check'
  | 'sun'
  | 'moon';

const icons: Record<IconName, NamedExoticComponent<{ className: string }>> = {
  'paper-plane': PaperPlaneIcon,
  'triangle-down': TriangleDownIcon,
  archive: ArchiveIcon,
  camera: CameraIcon,
  check: CheckIcon,
  gear: GearIcon,
  home: HomeIcon,
  update: UpdateIcon,
  sun: SunIcon,
  moon: MoonIcon,
  question: QuestionMarkIcon,
};

export interface IconProps {
  icon: IconName;
  className?: string;
  size?: 'md';
  spin?: boolean;
}

export function Icon({ icon, spin, size = 'md', className }: IconProps) {
  const Component = icons[icon] ?? icons.question;
  return (
    <Component
      className={classnames(
        className,
        'text-gray-800',
        size === 'md' && 'h-4 w-4',
        spin && 'animate-spin',
      )}
    />
  );
}
