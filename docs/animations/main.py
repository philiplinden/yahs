import numpy as np
from manim import (
    BLUE,
    GREEN,
    RED,
    WHITE,
    RIGHT,
    UP,
    DOWN,
    LEFT,
    Arrow,
    Axes,
    Circle,
    Dot,
    FadeIn,
    FadeOut,
    GrowArrow,
    Line,
    Scene,
    Square,
    Text,
    ShrinkToCenter,
    GrowFromPoint,
    GrowFromCenter,
    VGroup,
    ValueTracker,
    PI,
    TAU,
    VMobject,
    MoveAlongPath,
    there_and_back,
    rush_into,
    rush_from,
    linear,
    smooth,
    wiggle,
)


class Balloon(Circle):

    def __init__(self, *args, **kwargs):
        super().__init__(*args,
                         **kwargs,
                         radius=1,
                         color=RED,
                         fill_opacity=0.8)
        self.label = Text("Balloon", color=self.color).next_to(self, RIGHT)
        self.label.add_updater(lambda x: x.next_to(self, RIGHT))


class Tether(Line):

    def __init__(self, *args, **kwargs):
        super().__init__(
            *args,
            **kwargs,
            color="yellow",
        )
        self.label = Text("Tether", color=self.color).next_to(self, RIGHT)
        self.label.add_updater(lambda x: x.next_to(self, RIGHT))


class PayloadBox(Square):

    def __init__(self, *args, position=np.array([0, 0, 0]), **kwargs):
        super().__init__(
            *args,
            **kwargs,
            side_length=0.5,
            fill_color="#BB8E51",
            fill_opacity=1,
            stroke_width=0,
        )
        self.move_to(position)
        self.label = Text("Payload", color=self.color).next_to(self, RIGHT)
        self.label.add_updater(lambda x: x.next_to(self, RIGHT))


class BalloonAssembly:
    origin = np.array([0, 0, 0])
    payload_box_offset = np.array([0, -3, 0])
    dot = Dot(radius=0.1, color=RED)

    def __init__(self):
        self.balloon = Balloon()

        self.payload_box = PayloadBox(position=self.payload_box_offset)

        # Create a line from the bottom of the circle to the square
        self.tether = Tether(
            start=self.balloon.get_bottom(),
            end=self.payload_box.get_top(),
        )

    def get_objects(self):
        return self.balloon, self.payload_box, self.tether

    def fade_in_labels(self):
        return (FadeIn(obj.label) for obj in [
            self.balloon,
            self.tether,
            self.payload_box,
        ])

    def fade_out_labels(self):
        return (FadeOut(obj.label) for obj in [
            self.balloon,
            self.tether,
            self.payload_box,
        ])

    def collapse_to_dot(self):
        return (
            ShrinkToCenter(self.balloon),
            FadeOut(self.payload_box, scale=0.1, target_position=self.origin),
            FadeOut(self.tether, scale=0.1, target_position=self.origin),
            FadeIn(self.dot),
        )

    def expand_from_dot(self):
        self.balloon = Balloon(
        )  # for some reason the balloon needs to be recreated
        return (
            GrowFromCenter(self.balloon),
            GrowFromPoint(self.payload_box, self.origin),
            GrowFromPoint(self.tether, self.origin),
            FadeOut(self.dot),
        )


class ControlVolume(Scene):

    def construct(self):
        axes = Axes()
        axis_labels = axes.get_axis_labels(
            Text("x").scale(0.7),
            Text("y").scale(0.7))

        # Add the balloon assembly to the scene
        balloon_assembly = BalloonAssembly()
        balloon, payload_box, tether = balloon_assembly.get_objects()

        self.add(balloon, payload_box, tether)
        self.play(*balloon_assembly.fade_in_labels())
        self.wait(1)

        self.play(*balloon_assembly.fade_out_labels())

        control_volume_box = Square(
            side_length=5,
            color=BLUE,
            stroke_width=3,
            stroke_opacity=1,
            fill_opacity=0,
        )
        control_volume_box.move_to(np.array([0, -1, 0
                                             ]))  # Move the square down by 1.5
        control_volume_label = Text("Control Volume", color=BLUE).next_to(
            control_volume_box, UP)
        control_volume_label.add_updater(
            lambda x: x.next_to(control_volume_box, UP))

        self.play(FadeIn(axes), FadeIn(axis_labels),
                  FadeIn(control_volume_box), FadeIn(control_volume_label))
        self.wait(3)

        self.play(*balloon_assembly.collapse_to_dot())
        self.play(control_volume_box.animate.shift(UP * 1))
        self.play(control_volume_box.animate.scale(0.1))
        self.wait(1)

        # perfect loop
        self.play(FadeOut(axes), FadeOut(axis_labels),
                  FadeOut(control_volume_box), FadeOut(control_volume_label))
        self.play(*balloon_assembly.expand_from_dot())


class ForceBalance(Scene):

    def construct(self):
        # Create the balloon assembly
        balloon_assembly = BalloonAssembly()
        dot = balloon_assembly.dot

        # Add the balloon assembly to the scene
        self.add(*balloon_assembly.get_objects())
        self.play(*balloon_assembly.fade_in_labels())
        self.wait(1)

        self.play(*balloon_assembly.fade_out_labels(),
                  *balloon_assembly.collapse_to_dot(),
                  dot.animate.set_fill(WHITE))

        buoyancy_arrow = Arrow(dot.get_center(),
                               dot.get_center() + UP * 2,
                               color=GREEN)
        buoyancy_label = Text("Buoyancy",
                              color=GREEN).next_to(buoyancy_arrow, UP)
        buoyancy_label.add_updater(lambda x: x.next_to(buoyancy_arrow, UP))
        self.play(GrowArrow(buoyancy_arrow), FadeIn(buoyancy_label))

        gravity_arrow = Arrow(dot.get_center(),
                              dot.get_center() + DOWN * 2,
                              color=RED)
        gravity_label = Text("Weight", color=RED).next_to(gravity_arrow, DOWN)
        gravity_label.add_updater(lambda x: x.next_to(gravity_arrow, DOWN))
        self.play(GrowArrow(gravity_arrow), FadeIn(gravity_label))

        drag_arrow = Arrow(dot.get_center(), dot.get_center(), color=BLUE)
        drag_label = Text("Drag", color=BLUE).next_to(drag_arrow, RIGHT)
        drag_label.add_updater(lambda x: x.next_to(drag_arrow, RIGHT))

        group = VGroup(dot, buoyancy_arrow, buoyancy_label, gravity_arrow,
                       drag_arrow, drag_label)
        self.wait(2)

        # perfect loop
        self.play(
            dot.animate.set_fill(balloon_assembly.balloon.color),
            FadeOut(buoyancy_arrow, buoyancy_label),
            FadeOut(gravity_arrow, gravity_label),
            FadeOut(drag_arrow, drag_label),
        )
        self.play(*balloon_assembly.expand_from_dot())
