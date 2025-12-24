"use client";

import * as React from "react";
import { Clock2Icon } from "lucide-react";
import { format } from "date-fns";

import { Calendar } from "@/components/ui/calendar";
import { Card, CardContent, CardFooter } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface Field {
  value: string;
  onChange: (value: string) => void;
}

interface DateRangeTimePickerProps {
  startField: Field;
  endField: Field;
}

export function DateRangeTimePicker({
  startField,
  endField,
}: DateRangeTimePickerProps) {
  const startDate = startField.value ? new Date(startField.value) : new Date();

  const endDate = endField.value
    ? new Date(endField.value)
    : new Date(startDate.getTime() + 60 * 60 * 1000);

  const [date, setDate] = React.useState<Date>(startDate);
  const [startTime, setStartTime] = React.useState(format(startDate, "HH:mm"));
  const [endTime, setEndTime] = React.useState(format(endDate, "HH:mm"));

  const updateStart = (d: Date, time: string) => {
    const [h, m] = time.split(":").map(Number);
    const next = new Date(d);
    next.setHours(h, m, 0);
    startField.onChange(next.toISOString());
  };

  const updateEnd = (d: Date, time: string) => {
    const [h, m] = time.split(":").map(Number);
    const next = new Date(d);
    next.setHours(h, m, 0);
    endField.onChange(next.toISOString());
  };

  const handleDateSelect = (selected: Date | undefined) => {
    if (!selected) return;
    setDate(selected);
    updateStart(selected, startTime);
    updateEnd(selected, endTime);
  };

  return (
    <Card className="w-fit py-4">
      <CardContent className="px-4">
        <Calendar
          mode="single"
          selected={date}
          onSelect={handleDateSelect}
          className="bg-transparent p-2
            [--cell-size:--spacing(12)]
            md:[--cell-size:--spacing(14)]
          "
        />
      </CardContent>

      <CardFooter className="grid grid-cols-2 gap-4 border-t px-4 pt-4">
        {/* START */}
        <div className="flex flex-col gap-2">
          <Label>Start</Label>
          <div className="relative">
            <Clock2Icon className="absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="time"
              value={startTime}
              onChange={(e) => {
                setStartTime(e.target.value);
                updateStart(date, e.target.value);
              }}
              className="pl-8"
            />
          </div>
        </div>

        {/* END */}
        <div className="flex flex-col gap-2">
          <Label>End</Label>
          <div className="relative">
            <Clock2Icon className="absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              type="time"
              value={endTime}
              onChange={(e) => {
                setEndTime(e.target.value);
                updateEnd(date, e.target.value);
              }}
              className="pl-8"
            />
          </div>
        </div>
      </CardFooter>
    </Card>
  );
}
