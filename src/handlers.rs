use dbus_crossroads::Context;
use dbus_crossroads::IfaceBuilder;

use crate::data::Timetracker;

// TODO: find a way to pass empty arguments for getter
const NO_ARGUMENTS: (&str,) = ("__not_relevant__",);

pub fn build_timetracker_interface(builder: &mut IfaceBuilder<Timetracker>) {
  builder.signal::<(String,), _>("updated", ("sender",));
  builder.method(
    "get",
    NO_ARGUMENTS,
    ("status",),
    |_ctx: &mut Context, data: &mut Timetracker, (_name,): (String,)| {
      let rv = data.get_status();
      println!("Get called: {}", rv);
      Ok((rv,))
    },
  );
  builder.method(
    "set",
    ("value",),
    ("status",),
    |ctx: &mut Context, data: &mut Timetracker, (value,): (String,)| {
      data.set_status(&value);
      ctx.push_msg(ctx.make_signal("updated", (&value,)));
      println!("Set called: {}", &value);
      Ok((value.to_string(),))
    },
  );
}
