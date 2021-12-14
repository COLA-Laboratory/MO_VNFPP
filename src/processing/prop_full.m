addpath(".");

close all
clear
clc
format long g

%% Parameters
num_objectives = 3;

runs = 30;

% root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "Constraints", "Utilisation");
out_folder = fullfile(root_path, "processed", "utilisation");

if exist(out_folder, 'dir')
    rmdir(out_folder, 's')
end

alg_paths = [...
    fullfile("IBEA", "ca_operators"),...
    fullfile("IBEA", "standard_ga"),...
    fullfile("IBEA", "simple_model"),...
    fullfile("RoundRobin")
];

utilisations = dir(src_folder);
utilisations = natsortfiles({utilisations.name});
utilisations = utilisations(3:end); % Remove . and ..

for utilisation = utilisations
    for alg_path = alg_paths
        in_file = fullfile(src_folder, utilisation, alg_path, "aggregate.csv");
        in_file = csvread(in_file);
        
        mean = in_file(end, 2);
        stdev = in_file(end, 3);
        min = in_file(end, 4);
        lq = in_file(end, 5);
        median = in_file(end, 6);
        uq = in_file(end, 7);
        max = in_file(end, 8);
        
        out_part_folder = fullfile(out_folder, alg_path);
        out_file = fullfile(out_part_folder, 'final_hv.csv');
        
        if ~exist(out_part_folder, 'dir')
            mkdir(out_part_folder)
        end
        
        fid = fopen(out_file, 'a+');
        fprintf(fid, '%d,%f,%f,%f,%f,%f,%f,%f\n', str2double(utilisation), mean, stdev, min, lq, median, uq, max);
        fclose(fid);
    end
end